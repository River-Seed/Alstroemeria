#include <stdio.h>
#include "vulkanSetUp.hpp"

void mainLoop() {
  glfwInit();
  GLFWwindow* window;

  glfwWindowHint(GLFW_CLIENT_API, GLFW_NO_API);
  glfwWindowHint(GLFW_RESIZABLE, GLFW_FALSE);

  window = glfwCreateWindow(800, 600, "Alstroemeria", nullptr, nullptr);
  while(!glfwWindowShouldClose(window)) {
    glfwPollEvents();
  }
  cleanUp(window);
}

int main() {
  mainLoop();
  printf("Some words to say\n");
  return 0;
}
