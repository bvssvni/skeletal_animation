use std::cell::RefCell;
use std::rc::Rc;

use vecmath::{self, Matrix4};
use collada::Skeleton;
use gfx_debug_draw::DebugRenderer;
use gfx_device_gl::Resources as GlResources; // FIXME

pub fn draw_skeleton(skeleton: Rc<RefCell<Skeleton>>, global_poses: &[Matrix4<f32>], debug_renderer: &mut DebugRenderer<GlResources>, draw_labels: bool) {
    for (joint_index, joint) in skeleton.borrow().joints.iter().enumerate() {

        let joint_position = vecmath::row_mat4_transform(global_poses[joint_index], [0.0, 0.0, 0.0, 1.0]);

        let leaf_end = vecmath::row_mat4_transform(
            global_poses[joint_index],
            [0.0, 1.0, 0.0, 1.0]
        );

        if !joint.is_root() {
            let parent_position = vecmath::row_mat4_transform(global_poses[joint.parent_index as usize], [0.0, 0.0, 0.0, 1.0]);

            // Draw bone (between joint and parent joint)

            debug_renderer.draw_line(
                [parent_position[0], parent_position[1], parent_position[2]],
                [joint_position[0], joint_position[1], joint_position[2]],
                [0.2, 0.2, 0.2, 1.0]
            );

            if !skeleton.borrow().joints.iter().any(|j| j.parent_index as usize == joint_index) {

                // Draw extension along joint's y-axis...
                debug_renderer.draw_line(
                    [joint_position[0], joint_position[1], joint_position[2]],
                    [leaf_end[0], leaf_end[1], leaf_end[2]],
                    [0.2, 0.2, 0.2, 1.0]
                );
            }
        }

        if draw_labels {
            // Label joint
            debug_renderer.draw_text_at_position(
                &joint.name[..],
                [leaf_end[0], leaf_end[1], leaf_end[2]],
                [1.0, 1.0, 1.0, 1.0]
            );
        }

        // Draw joint-relative axes
        let p_x_axis = vecmath::row_mat4_transform(
            global_poses[joint_index],
            [1.0, 0.0, 0.0, 1.0]
        );

        let p_y_axis = vecmath::row_mat4_transform(
            global_poses[joint_index],
            [0.0, 1.0, 0.0, 1.0]
        );

        let p_z_axis = vecmath::row_mat4_transform(
            global_poses[joint_index],
            [0.0, 0.0, 1.0, 1.0]
        );

        debug_renderer.draw_line(
            [joint_position[0], joint_position[1], joint_position[2]],
            [p_x_axis[0], p_x_axis[1], p_x_axis[2]],
            [1.0, 0.2, 0.2, 1.0]
        );

        debug_renderer.draw_line(
            [joint_position[0], joint_position[1], joint_position[2]],
            [p_y_axis[0], p_y_axis[1], p_y_axis[2]],
            [0.2, 1.0, 0.2, 1.0]
        );

        debug_renderer.draw_line(
            [joint_position[0], joint_position[1], joint_position[2]],
            [p_z_axis[0], p_z_axis[1], p_z_axis[2]],
            [0.2, 0.2, 1.0, 1.0]
        );
    }
}