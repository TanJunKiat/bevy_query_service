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

#[derive(Component, Clone)]
struct NameRequest;

#[derive(Component, Clone, Default, Debug)]
struct ParentComponent{last_name: String}

#[derive(Component, Clone, Default, Debug)]
struct ChildComponent{first_name: String}

#[derive(Component, Clone, Default, Debug)]
struct FullName(String);

impl QueryReplyOpsTriple<NameRequest, ParentComponent, ChildComponent> for FullName {
    fn get_reply(_request: &QueryRequest<NameRequest>, parent_queries: &Query<(Entity, &ParentComponent), With<ParentComponent>>, child_queries: &Query<(&bevy_hierarchy::Parent, Entity, &ChildComponent), With<ChildComponent>>) -> Result<Self,()> {
        
        for (parent_entity, _, child_component) in child_queries.iter() {
            match parent_queries.get(**parent_entity){
                Ok((_, parent_component)) => {
                    return Ok(FullName(format!("{} {}", child_component.first_name, parent_component.last_name)));
                },
                Err(_) => {
                    return Err(());
                }
            }
        }
        return Err(());
    }
}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);

    app.add_plugins(QueryServicePlugin);
    app.add_systems(Startup, spawn_parent_child);

    app.add_event::<QueryEvent<NameRequest>>();
    app.add_systems(Update, spawn_request_endpoint::<NameRequest, FullName>);
    app.add_systems(Update, run_query_server_with_parent::<NameRequest, FullName, ParentComponent, ChildComponent>);
    app.add_systems(Update, cleanup_requests);

    app.add_plugins(EguiPlugin);
    app.add_systems(Update, interaction_panel);
    app.add_systems(Update, request_panel);
    app.run();
}

fn spawn_parent_child(mut commands: Commands){
    let parent = commands.spawn((ParentComponent{last_name: "Smith".into()},)).id();
    let child = commands.spawn((ChildComponent{first_name: "John".into()},)).id();
    commands.entity(parent).add_child(child);
}

fn interaction_panel(mut contexts: EguiContexts, mut query_event_writer: EventWriter<QueryEvent<NameRequest>>) {
    let ctx = contexts.ctx_mut();

    egui::SidePanel::left("left_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.label("Interaction");
            ui.horizontal(|ui| {
                if ui.button("Send query request").clicked() {
                    query_event_writer.send(QueryEvent {
                        uuid: uuid::Uuid::new_v4(),
                        request: NameRequest,
                    });
                }
            });
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width();
}

fn request_panel(mut contexts: EguiContexts, mut reply_queries: Query<(&mut GoalComponent, &QueryReply<FullName>), With<QueryReply<FullName>>>) {
    let ctx = contexts.ctx_mut();

    egui::SidePanel::right("right_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.label("Requests");
            for (mut goal, query) in reply_queries.iter_mut() {
                ui.horizontal(|ui| {
                    ui.label(format!("Full name: {:?}", query.reply.0));
                    if ui.button("Delete").clicked() {
                        info!("Deleting request...");
                        goal.to_delete = true;
                    }
                });
            }
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width();
}
