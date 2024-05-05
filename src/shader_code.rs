pub static VERTEX_SHADER: &str = "
            #version 460 core\n
            layout (location = 0) in vec3 aPos;
            layout (location = 1) in vec2 aUv;

            out vec2 uv;
            void main()
            {
                uv = aUv;
                gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
            }
            \0";

pub static PIXEL_SHADER: &str = "
            #version 460 core\n
            uniform ivec2 cursor_pos;
            uniform ivec2 screen_size;
            uniform int brush_size;
            uniform sampler2D tex;

            in vec2 uv;

            out vec4 pixelColor;
            void main()
            {
                pixelColor = length(cursor_pos - uv * screen_size) < brush_size &&
                             length(cursor_pos - uv * screen_size) > brush_size - 1 ? vec4(1) : texture(tex, uv);
            }
            \0";

pub static COMPUTE_SHADER: &str = "
                #version 460\n
layout (local_size_x = 10, local_size_y = 10, local_size_z = 1) in;

layout(rgba32f, binding = 0) uniform image2D imgOutput;
uniform ivec2 cursor_pos;
uniform ivec2 prev_cursor_pos;
// uniform ivec2 screen_size;
uniform int brush_size;

uniform vec4 color;

void main() {
    //vec2 uv = vec2(0.0, 0.0);
    int minX = min(prev_cursor_pos.x, cursor_pos.x);
    int minY = min(prev_cursor_pos.y, cursor_pos.y);
    ivec2 screen_coord = ivec2(gl_GlobalInvocationID.xy + ivec2(minX - min(brush_size, minX), minY - min(brush_size, minY)));

    //uv.x = float(screen_coord.x) / (gl_NumWorkGroups.x * gl_WorkGroupSize.x);
    //uv.y = float(screen_coord.y) / (gl_NumWorkGroups.y * gl_WorkGroupSize.y);

    //ivec2 pixel = ivec2(uv.x * screen_size.x, uv.y * screen_size.y);

    // Draw
    vec2 dir = vec2(normalize(prev_cursor_pos - cursor_pos));

    ivec2 left_down = cursor_pos - ivec2(vec2(-dir.y, dir.x) * brush_size);
    ivec2 left_up = cursor_pos + ivec2(vec2(-dir.y, dir.x) * brush_size);
    ivec2 right_down = prev_cursor_pos - ivec2(vec2(-dir.y, dir.x) * brush_size);

    ivec2 p1 = screen_coord - right_down;
    ivec2 p2 = right_down - left_down;

    ivec2 p3 = screen_coord - left_up;
    ivec2 p4 = left_up - left_down;

    if ((
    p1.x * p2.y - p1.y * p2.x < 0 &&
    p1.x * p4.y - p1.y * p4.x < 0 &&
    p2.x * p3.y - p2.y * p3.x < 0 &&
    p3.x * p4.y - p3.y * p4.x > 0)||
    length(screen_coord - cursor_pos) < brush_size
    )
    {
        imageStore(imgOutput, screen_coord, color);
    }
}
\0";