// =========================================================================
/*
 * Copyright (C) 2019 Tan Jun Kiat
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
*/
// =========================================================================
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_query_service::*;
use anyhow::Result;

#[derive(Component, Clone)]
struct Ping;

#[derive(Component, Clone, Default, Debug)]
struct Pong(bool);

impl QueryServerOps<Ping> for Pong {
    fn get_reply(_world: &mut World, _request: &QueryRequest<Ping>) -> Result<Self> {
        Ok(Pong(true))
    }
}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);

    app.add_plugins(QueryServicePlugin);

    app.add_event::<QueryEvent<Ping>>();
    app.add_systems(Update, spawn_request_endpoint::<Ping, Pong>);
    app.add_systems(Update, run_query_server::<Ping, Pong>);
    app.add_systems(Update, cleanup_requests);

    app.add_plugins(EguiPlugin);
    app.add_systems(Update, interaction_panel);
    app.add_systems(Update, request_panel);
    app.run();
}

fn interaction_panel(mut contexts: EguiContexts, mut query_event_writer: EventWriter<QueryEvent<Ping>>) {
    let ctx = contexts.ctx_mut();

    egui::SidePanel::left("left_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.label("Interaction");
            ui.horizontal(|ui| {
                if ui.button("Send query request").clicked() {
                    query_event_writer.send(QueryEvent {
                        uuid: uuid::Uuid::new_v4(),
                        request: Ping,
                    });
                }
            });
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width();
}

fn request_panel(mut contexts: EguiContexts, mut reply_queries: Query<(&mut GoalComponent, &QueryReply<Pong>), With<QueryReply<Pong>>>) {
    let ctx = contexts.ctx_mut();

    egui::SidePanel::right("right_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.label("Requests");
            for (mut goal, query) in reply_queries.iter_mut() {
                ui.horizontal(|ui| {
                    ui.label(format!("Request: {:?}", query.reply.0));
                    if ui.button("Delete").clicked() {
                        info!("Deleting request...");
                        goal.mark_to_delete();
                    }
                });
            }
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width();
}
