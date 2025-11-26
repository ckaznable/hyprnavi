use std::cmp;

use hyprland::{
    data::{Client, Clients},
    dispatch::{
        Direction, Dispatch, DispatchType, WindowIdentifier, WindowMove,
        WorkspaceIdentifierWithSpecial,
        Direction, Dispatch, DispatchType, WindowIdentifier, WorkspaceIdentifierWithSpecial,
    },
    shared::{HyprData, HyprDataActiveOptional, HyprDataVec},
};

use crate::cli::{Command, Flags};

mod cli;

fn main() -> anyhow::Result<()> {
    let params: Flags = argh::from_env();

    let clients = Clients::get()?;
    assert!(clients.iter().len() > 1, "less than 2 clients");

    let Some(act_client) = Client::get_active().ok().flatten() else {
        return handle_in_empty_ws(params);
    };
    let act_ws_id = act_client.workspace.id;

    let clients = clients.to_vec();
    let first_client = clients.first().unwrap();
    let last_client = clients.last().unwrap();
    let (prev_ws_id, next_ws_id) = get_neighborhood_workspace(&clients, act_ws_id);
    let (left, right) = get_bound_client(&clients, act_ws_id).unwrap();

    use Command::*;
    match params.cmd {
        Next(params) => {
            if params.no_wrap
                && is_bound(&act_client, right, true)
                && is_wrapping_boundary(&clients, act_ws_id, next_ws_id, true)
            {
                return Ok(());
            }

            let next_left = get_bound_client(&clients, next_ws_id).map_or(first_client, |(c, _)| c);

            if is_bound(&act_client, right, true) {
                handle_bound_navigation(next_left, &act_client, params.swap, params.move_window)?;
            } else {
                handle_default_navigation(Direction::Right, params.swap, params.move_window)?;
            }
        }
        Prev(params) => {
            if params.no_wrap
                && is_bound(&act_client, left, false)
                && is_wrapping_boundary(&clients, act_ws_id, prev_ws_id, false)
            {
                return Ok(());
            }

            let prev_right = get_bound_client(&clients, prev_ws_id).map_or(last_client, |(_, c)| c);

            if is_bound(&act_client, left, false) {
                handle_bound_navigation(prev_right, &act_client, params.swap, params.move_window)?;
            } else {
                handle_default_navigation(Direction::Left, params.swap, params.move_window)?
            };
        }
    };

    Ok(())
}

fn handle_in_empty_ws(params: Flags) -> anyhow::Result<()> {
    use Command::*;
    Dispatch::call(match params.cmd {
        Next(_) => DispatchType::Custom("workspace", "e+1"),
        Prev(_) => DispatchType::Custom("workspace", "e-1"),
    })?;

    Ok(())
}

#[inline]
fn handle_default_navigation(dir: Direction, swap: bool, move_window: bool) -> anyhow::Result<()> {
    Dispatch::call(if swap {
        DispatchType::SwapWindow(dir)
    } else if move_window {
        DispatchType::MoveWindow(WindowMove::Direction(dir))
    } else {
        DispatchType::MoveFocus(dir)
    })?;

    Ok(())
}

fn handle_move_window_to_ws(act_client: &Client, target_ws_id: i32) -> anyhow::Result<()> {
    // Use non-silent move to workspace to move client and focus new workspace
    Dispatch::call(DispatchType::MoveToWorkspace(
        WorkspaceIdentifierWithSpecial::Id(target_ws_id),
        Some(WindowIdentifier::Address(act_client.address.clone())),
    ))?;
    Ok(())
}

fn handle_bound_navigation(
    client: &Client,
    act_client: &Client,
    swap: bool,
    move_window: bool,
) -> anyhow::Result<()> {
    if swap {
        handle_swap(client, act_client)
    } else if move_window {
        handle_move_window_to_ws(act_client, client.workspace.id)
    } else {
        handle_focus(client)
    }
}

fn handle_swap(client: &Client, act_client: &Client) -> anyhow::Result<()> {
    // move current to target workspace
    Dispatch::call(DispatchType::MoveToWorkspaceSilent(
        WorkspaceIdentifierWithSpecial::Id(client.workspace.id),
        Some(WindowIdentifier::Address(act_client.address.clone())),
    ))?;

    // move target client to current workspace
    Dispatch::call(DispatchType::MoveToWorkspace(
        WorkspaceIdentifierWithSpecial::Id(act_client.workspace.id),
        Some(WindowIdentifier::Address(client.address.clone())),
    ))?;

    Ok(())
}

fn handle_focus(client: &Client) -> anyhow::Result<()> {
    Dispatch::call(DispatchType::FocusWindow(WindowIdentifier::Address(
        client.address.clone(),
    )))?;
    Ok(())
}

#[inline]
fn get_neighborhood_workspace(clients: &[Client], act_ws_id: i32) -> (i32, i32) {
    let near_than_last = |id_cur: i32, id_last: i32| -> bool {
        let dist_cur = (id_cur - act_ws_id).abs();
        let dist_last = (id_last - act_ws_id).abs();
        dist_cur != 0 && (dist_last == 0 || dist_last > dist_cur)
    };

    let (prev, next, max, min) = clients
        .iter()
        .filter(|client| {
            client.workspace.id != act_ws_id && !client.workspace.name.starts_with("special")
        })
        .fold(
            (act_ws_id, act_ws_id, act_ws_id, act_ws_id),
            |acc, client| {
                let id = client.workspace.id;
                let prev = if act_ws_id > id && near_than_last(id, acc.0) {
                    id
                } else {
                    acc.0
                };
                let next = if act_ws_id < id && near_than_last(id, acc.1) {
                    id
                } else {
                    acc.1
                };
                (prev, next, cmp::max(acc.2, id), cmp::min(acc.3, id))
            },
        );

    let prev = if prev == act_ws_id { max } else { prev };
    let next = if next == act_ws_id { min } else { next };
    (prev, next)
}

#[inline]
fn is_wrapping_boundary(
    clients: &[Client],
    act_ws_id: i32,
    neighbor_ws_id: i32,
    direction_right: bool,
) -> bool {
    let ws_ids: Vec<i32> = clients
        .iter()
        .filter(|c| !c.workspace.name.starts_with("special"))
        .map(|c| c.workspace.id)
        .collect();

    if ws_ids.is_empty() {
        return false;
    }

    let min_ws_id = *ws_ids.iter().min().unwrap();
    let max_ws_id = *ws_ids.iter().max().unwrap();

    // Wrap to the right (Next) means jumping from MAX to MIN.
    if direction_right {
        return neighbor_ws_id == min_ws_id && act_ws_id == max_ws_id;
    }
    // Wrap to the left (Prev) means jumping from MIN to MAX.
    else {
        return neighbor_ws_id == max_ws_id && act_ws_id == min_ws_id;
    }
}

fn get_bound_client(clients: &[Client], workspace: i32) -> Option<(&Client, &Client)> {
    if clients.is_empty() {
        return None;
    }

    if clients.len() == 1 {
        return Some((&clients[0], &clients[0]));
    }

    let cmp_left = |a: &Client, b: &Client| a.at.0.cmp(&b.at.0).is_gt();
    let cmp_right = |a: &Client, b: &Client| {
        let a = a.at.0 + a.size.0;
        let b = b.at.0 + b.size.0;
        a.cmp(&b).is_le()
    };

    let result = clients
        .iter()
        .filter(|client| {
            client.workspace.id == workspace && !client.workspace.name.starts_with("special")
        })
        .fold((&clients[0], &clients[0]), |mut result, client| {
            if result.0.workspace.id != workspace {
                result.0 = client;
            }

            if result.1.workspace.id != workspace {
                result.1 = client;
            }

            if cmp_left(result.0, client) {
                result.0 = client;
            }

            if cmp_right(result.1, client) {
                result.1 = client;
            }

            result
        });

    Some(result)
}

#[inline]
fn is_bound(act: &Client, right: &Client, side_right: bool) -> bool {
    act.address == right.address
        || ((side_right && act.at.0 + act.size.0 == right.at.0 + right.size.0)
            || act.at.0 == right.at.0)
}
