#version 330 core
out vec4 FragColor;
in vec2 TexCoord;
uniform sampler2D texture1;
uniform sampler2D texture2;
uniform float transparency;

void main()
{
    vec4 box = texture(texture1, TexCoord);
    vec4 moai = texture(texture2, TexCoord);
    float alpha = moai.a * transparency;
    FragColor = vec4(box.xyz * (1 - alpha) + moai.xyz * alpha, 1);
}
