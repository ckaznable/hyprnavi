use std::cmp;

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
    let near_than_last = |id_cur: i32, id_last: i32| -> bool {
        let dist_cur = (id_cur - act_ws_id).abs();
        let dist_last = (id_last - act_ws_id).abs();
        dist_cur != 0 && (dist_last == 0 || dist_last > dist_cur)
    };

    let (prev, next, max, min) = clients
        .iter()
        .filter(|client| client.workspace.id != act_ws_id)
        .fold((act_ws_id, act_ws_id, act_ws_id, act_ws_id), |acc, client| {
            let id = client.workspace.id;
            let prev = if act_ws_id > id && near_than_last(id, acc.0) { id } else { acc.0 };
            let next = if act_ws_id < id && near_than_last(id, acc.1) { id } else { acc.1 };
            (prev, next, cmp::max(acc.2, id), cmp::min(acc.3, id))
        });

    let prev = if prev == act_ws_id { max } else { prev };
    let next = if next == act_ws_id { min } else { next };
    (prev, next)
}

fn get_edge_client(clients: &[Client], workspace: i32) -> Option<(&Client, &Client)> {
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

    let clients_in_ws = clients
        .iter()
        .filter(|client| client.workspace.id == workspace)
        .collect::<Vec<&Client>>();

    let result = clients_in_ws
        .iter()
        .fold((clients_in_ws[0], clients_in_ws[0]), |mut result, client| {
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
