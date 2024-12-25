#version 330 core
out vec4 FragColor;
in vec2 TexCoord;
uniform sampler2D texture1;
uniform sampler2D texture2;

void main()
{
    vec4 box = texture(texture1, TexCoord);
    vec4 moai = texture(texture2, vec2(1 - TexCoord.x, TexCoord.y));
    FragColor = mix(box, moai, 0.2);
}
