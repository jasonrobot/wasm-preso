function strlen(ptr) {
  // Bitwise OR with 0 converts this value to a 32-bit signed int
  ptr = ptr|0;
  var curr = 0;
  curr = ptr;
  // MEM8 is an a Uint8Array in Javascript, which is just an array of bytes
  while ((MEM8[curr>>0]|0) != 0) {
    curr = (curr + 1)|0;
  }
  return (curr - ptr)|0;
}
