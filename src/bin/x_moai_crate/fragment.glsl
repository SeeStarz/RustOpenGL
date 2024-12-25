#version 330 core
out vec4 FragColor;
in vec2 TexCoord;
uniform float transparency;
uniform sampler2D texture1;
uniform sampler2D texture2;

void main()
{
    vec4 box = texture(texture1, TexCoord);
    vec4 moai = texture(texture2, TexCoord);
    float trans = ((sin(transparency) + 1) / 2) * moai.a;
    FragColor = vec4(box.rgb * (1 - trans) + moai.rgb * trans, 1);
}
