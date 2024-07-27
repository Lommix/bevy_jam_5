#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput
#import bevy_render::view::View

@group(0) @binding(0) var source_tex: texture_2d<f32>;
@group(0) @binding(1) var source_sampler: sampler;

@group(0) @binding(2) var highlight_tex: texture_2d<f32>;
@group(0) @binding(3) var highlight_sampler: sampler;

@group(0) @binding(4) var<uniform> uni: Uniform;

struct Uniform {
	glow_thickness: f32,
	// add highlight color
}

@fragment
fn fragment(in : FullscreenVertexOutput) -> @location(0) vec4<f32>{
	var out : vec4<f32>;

	let main_pixel = textureSample(source_tex, source_sampler, in.uv);
	let highlight_pixel = textureSample(highlight_tex, highlight_sampler, in.uv);

	let screen_size = vec2<f32>(textureDimensions(source_tex));
	let border_thickness = 0.005;

	let left = textureSample(highlight_tex, highlight_sampler, in.uv + border_thickness * vec2(-1.,0.));
	let right = textureSample(highlight_tex, highlight_sampler, in.uv + border_thickness * vec2(1.,0.));
	let top = textureSample(highlight_tex, highlight_sampler, in.uv + border_thickness * vec2(0.,-1.));
	let down = textureSample(highlight_tex, highlight_sampler, in.uv + border_thickness * vec2(0.,1.));

	var border = ( left.a + right.a + top.a + down.a ) * ( 1. - highlight_pixel.a );

	out = select(main_pixel, highlight_pixel, highlight_pixel.a > 0.1);
	out = mix(out, vec4(1.), border);

	return out;
}
