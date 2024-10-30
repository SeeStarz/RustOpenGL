#version 330 core
out vec4 FragColor;

uniform vec2 windowSize;
uniform float time;
void main()
{
    const float PI = 3.14159265359;
    vec2 pos = gl_FragCoord.xy * 2 / windowSize - 1;
    float dist = distance(pos, vec2(0));
    float modif = step(0, 1 - dist);

    float h = mod(atan(pos.y, pos.x) + time, PI * 2); // From 0 to 2pi
    float s = dist;
    float v = 1; // Color wheel is ugly if value isn't one

    float c = v * s;
    float x = c * (1 - abs(mod(h * 3 / PI, 2) - 1));
    float m = v - c;

    vec3 rgb;
    if (h < PI / 3)
        rgb = vec3(c, x, 0);
    else if (h < 2 * PI / 3)
        rgb = vec3(x, c, 0);
    else if (h < PI)
        rgb = vec3(0, c, x);
    else if (h < 4 * PI / 3)
        rgb = vec3(0, x, c);
    else if (h < 5 * PI / 3)
        rgb = vec3(x, 0, c);
    else
        rgb = vec3(c, 0, x);
    rgb = rgb + vec3(m);

    FragColor = vec4(modif * rgb, 1);
}
