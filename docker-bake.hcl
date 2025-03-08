variable "IMAGE" {
  default = "ghcr.io/AlphaSphereDotAI/chatacter_backend_voice_generator"
}
target "default" {
  name = "chatacter_backend_voice_generator"
	context = .
  tags = [ "${IMAGE}:dev", "${IMAGE}:latest" ]
  dockerfile = "Dockerfile"
  platforms = [ "linux/amd64", "linux/arm64" ]
}
