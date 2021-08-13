shader_type canvas_item;
const float FG_MASK_ALPHA = 1.0;
const float BG_MASK_ALPHA = 0.0;

uniform vec4 fg : hint_color = vec4(1.0);
uniform vec4 bg : hint_color = vec4(vec3(0.0), 1.0);

void fragment() {
//	if step(alpha, FG_MASK_ALPHA)
//	COLOR = step(alpha, FG_MASK_ALPHA) * fg
	if (texture(TEXTURE, UV).a > 0.5) {
		COLOR = fg;
	} else {
		COLOR = bg;
	}
}