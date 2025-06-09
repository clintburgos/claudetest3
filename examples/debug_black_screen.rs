use bevy::prelude::*;
use claudetest3::{game::GameState, ui};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.4, 0.4, 0.4)))
        .add_plugins((
            game::GameStatePlugin,
            ui::world::WorldPlugin,
            ui::panels::UIPanelsPlugin,
        ))
        .add_systems(Update, debug_ui_hierarchy)
        .run();
}

fn debug_ui_hierarchy(
    query: Query<(Entity, &Node, Option<&BackgroundColor>, Option<&Name>)>,
    children: Query<&Children>,
    state: Res<State<GameState>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::KeyD) {
        info!("=== Current State: {:?} ===", state.get());
        info!("=== UI Hierarchy Debug ===");
        
        // Find root nodes (nodes without parents in the UI hierarchy)
        let mut root_nodes = Vec::new();
        for (entity, node, _, _) in query.iter() {
            let mut has_ui_parent = false;
            for (_, _, _, _) in query.iter() {
                if let Ok(entity_children) = children.get(entity) {
                    if entity_children.contains(&entity) {
                        has_ui_parent = true;
                        break;
                    }
                }
            }
            if !has_ui_parent && node.position_type == PositionType::Absolute {
                root_nodes.push(entity);
            }
        }
        
        // Print hierarchy for each root
        for root in root_nodes {
            print_node_hierarchy(&query, &children, root, 0);
        }
        
        info!("=== End UI Debug ===");
    }
}

fn print_node_hierarchy(
    query: &Query<(Entity, &Node, Option<&BackgroundColor>, Option<&Name>)>,
    children: &Query<&Children>,
    entity: Entity,
    depth: usize,
) {
    if let Ok((_, node, bg_color, name)) = query.get(entity) {
        let indent = "  ".repeat(depth);
        let name_str = name.map(|n| n.as_str()).unwrap_or("Unnamed");
        
        let size_info = format!(
            "{}x{}", 
            match node.width {
                Val::Percent(p) => format!("{}%", p),
                Val::Px(px) => format!("{}px", px),
                _ => "auto".to_string(),
            },
            match node.height {
                Val::Percent(p) => format!("{}%", p),
                Val::Px(px) => format!("{}px", px),
                _ => "auto".to_string(),
            }
        );
        
        let bg_info = bg_color
            .map(|c| format!(" BG: {:?}", c.0))
            .unwrap_or_default();
        
        info!("{}{:?} [{}] {} {}", indent, entity, name_str, size_info, bg_info);
        
        if let Ok(entity_children) = children.get(entity) {
            for child in entity_children.iter() {
                print_node_hierarchy(query, children, *child, depth + 1);
            }
        }
    }
}