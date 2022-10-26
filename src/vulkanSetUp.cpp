#include "GLFW/glfw3.h"
#include "vulkanSetUp.hpp"

void cleanUp(GLFWwindow* window) {
  glfwDestroyWindow(window);
  glfwTerminate();
}
