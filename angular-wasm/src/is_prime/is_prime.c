#include <emscripten.h>

int EMSCRIPTEN_KEEPALIVE is_prime(int number){
  if (number < 2 || number % 2 == 0) {
    return 0;
  }

  for (int i = 3; (i * i) <= number; i += 2) {
    if (number % i == 0) {
      return 0;
    }
  }

  return 1;
}
