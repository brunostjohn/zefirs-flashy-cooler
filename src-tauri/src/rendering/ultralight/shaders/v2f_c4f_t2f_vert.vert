#version 330

// Program Uniforms
uniform vec4 State;
uniform mat4 Transform;
uniform Scalar {
    float scalar[8];
};
uniform Clip {
    mat4 clip[8];
};
uniform Vector {
    vec4 vector[8];
};
uniform uint ClipSize;

// Uniform Accessor Functions
float Time() { return State[0]; }
float ScreenWidth() { return State[1]; }
float ScreenHeight() { return State[2]; }
float ScreenScale() { return State[3]; }
vec4 sRGBToLinear(vec4 val) { return vec4(val.xyz * (val.xyz * (val.xyz * 0.305306011 + 0.682171111) + 0.012522878), val.w); }

// Vertex Attributes
in vec2 in_Position;
in vec4 in_Color;
in vec2 in_TexCoord;

// Out Params
out vec4 ex_Color;
out vec2 ex_ObjectCoord;
out vec2 ex_ScreenCoord;

void main(void)
{
  ex_ObjectCoord = in_TexCoord;
  gl_Position = Transform * vec4(in_Position, 0.0, 1.0);
  ex_Color = in_Color;
}