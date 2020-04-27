#version 330 core
out vec4 FragColor;

in vec3 WorldPos;
in vec3 ourColor;
in vec2 TexCoord;
in vec3 Normal;

uniform sampler2D texture1;
uniform vec3 lightPos;
uniform vec3 lightColor;

void main()
{
	// ambient
	float ambientStrength = 0.1;
	vec3 ambient = ambientStrength * lightColor;

	// diffuse
	vec3 norm = normalize(Normal);
	vec3 lightDir = normalize(lightPos - WorldPos);
	float diff = max(dot(norm, lightDir), 0.0);
	vec3 diffuse = diff * lightColor;

	FragColor = vec4(ambient + diffuse, 1.0) * texture(texture1, TexCoord);
}
