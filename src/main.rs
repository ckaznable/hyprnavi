use std::cmp::Ordering;

use hyprland::{
    data::{Client, Clients},
    dispatch::{Direction, Dispatch, DispatchType, WindowIdentifier},
    shared::{HyprData, HyprDataActiveOptional, HyprDataVec},
};

use crate::cli::{Command, Flags};

mod cli;

fn main() -> anyhow::Result<()> {
    let params: Flags = argh::from_env();

    let clients = Clients::get()?;
    assert!(clients.iter().len() > 1, "less than 2 clients");

    let act_client = Client::get_active()
        .ok()
        .flatten()
        .expect("active client not exist");
    let act_ws_id = act_client.workspace.id;

    let clients = clients.to_vec();
    let first_client = clients.first().unwrap();
    let last_client = clients.last().unwrap();
    let (prev_ws_id, next_ws_id) = get_neighborhood_workspace(&clients, act_ws_id);
    let (left, right) = get_edge_client(&clients, act_ws_id).unwrap();

    let dispatch_type = match params.cmd {
        Command::Next(_) => {
            if right.address == act_client.address {
                let (next_left, _) = match get_edge_client(&clients, next_ws_id) {
                    Some(c) => c,
                    None => (first_client, first_client),
                };

                DispatchType::FocusWindow(WindowIdentifier::Address(next_left.address.clone()))
            } else {
                DispatchType::MoveFocus(Direction::Right)
            }
        }
        Command::Prev(_) => {
            if left.address == act_client.address {
                let (_, prev_right) = match get_edge_client(&clients, prev_ws_id) {
                    Some(c) => c,
                    None => (last_client, last_client),
                };

                DispatchType::FocusWindow(WindowIdentifier::Address(prev_right.address.clone()))
            } else {
                DispatchType::MoveFocus(Direction::Left)
            }
        }
    };

    Dispatch::call(dispatch_type)?;
    Ok(())
}

#[inline]
fn get_neighborhood_workspace(clients: &[Client], act_ws_id: i32) -> (i32, i32) {
    clients.iter().fold((act_ws_id, act_ws_id), |acc, client| {
        let id = client.workspace.id;
        if id == act_ws_id {
            return acc;
        }

        let diff = act_ws_id - id;
        let diff_last = act_ws_id - acc.0;
        let prev = if diff < diff_last && diff != 0 {
            id
        } else {
            acc.0
        };

        let diff = id - act_ws_id;
        let diff_last = acc.0 - act_ws_id;
        let next = if diff < diff_last && diff != 0 {
            id
        } else {
            acc.1
        };

        (prev, next)
    })
}

fn get_edge_client(clients: &[Client], workspace: i32) -> Option<(&Client, &Client)> {
    if clients.is_empty() {
        return None;
    }

    if clients.len() == 1 {
        return Some((&clients[0], &clients[0]));
    }

    let result = clients
        .iter()
        .filter(|client| client.workspace.id == workspace)
        .fold((&clients[0], &clients[0]), |mut result, client| {
            if cmp_left(result.0, client).is_gt() {
                result.0 = client;
            }

            if cmp_right(result.1, client).is_le() {
                result.1 = client;
            }

            result
        });

    Some(result)
}

#[inline]
fn cmp_left(a: &Client, b: &Client) -> Ordering {
    a.at.0.cmp(&b.at.0)
}

#[inline]
fn cmp_right(a: &Client, b: &Client) -> Ordering {
    let a = a.at.0 + a.size.0;
    let b = b.at.0 + b.size.0;
    a.cmp(&b)
}
