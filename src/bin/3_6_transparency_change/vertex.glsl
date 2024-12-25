#version 330 core
layout(location = 0) in vec2 pos;
layout(location = 1) in vec2 aTexCoord;
out vec2 TexCoord;

void main()
{
    TexCoord = aTexCoord;
    gl_Position = vec4(pos.xy, 1.0, 1.0);
}