/* This file uses a C++11 feature only for type-checking with Apple Clang. */

#include <memory>

extern "C"
void cxx11_clang() {
  std::unique_ptr<int> p;
}
