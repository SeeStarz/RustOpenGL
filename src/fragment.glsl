#version 330 core
out vec4 FragColor;
in vec2 TexCoord;
uniform float transparency;
uniform sampler2D texture1;
uniform sampler2D texture2;

void main()
{
    float trans = (sin(transparency * 5) + 1) / 2;
    FragColor = mix(texture(texture1, TexCoord), texture(texture2, 1 - TexCoord), trans);
}
