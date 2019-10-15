// Copyright (c) 2019, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

// Copyright © 2018, VideoLAN and dav1d authors
// Copyright © 2018, Two Orioles, LLC
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice, this
//    list of conditions and the following disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice,
//    this list of conditions and the following disclaimer in the documentation
//    and/or other materials provided with the distribution.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
// ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
// WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE LIABLE FOR
// ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
// (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
// LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
// ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
// SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use crate::util::AlignedArray;

#[rustfmt::skip]
#[no_mangle]
pub static rav1e_mc_subpel_filters: AlignedArray<[i8; 600]> = AlignedArray::new([
   0,   1,  -3,  63,   4,  -1,   0,   0, // REGULAR
   0,   1,  -5,  61,   9,  -2,   0,   0,
   0,   1,  -6,  58,  14,  -4,   1,   0,
   0,   1,  -7,  55,  19,  -5,   1,   0,
   0,   1,  -7,  51,  24,  -6,   1,   0,
   0,   1,  -8,  47,  29,  -6,   1,   0,
   0,   1,  -7,  42,  33,  -6,   1,   0,
   0,   1,  -7,  38,  38,  -7,   1,   0,
   0,   1,  -6,  33,  42,  -7,   1,   0,
   0,   1,  -6,  29,  47,  -8,   1,   0,
   0,   1,  -6,  24,  51,  -7,   1,   0,
   0,   1,  -5,  19,  55,  -7,   1,   0,
   0,   1,  -4,  14,  58,  -6,   1,   0,
   0,   0,  -2,   9,  61,  -5,   1,   0,
   0,   0,  -1,   4,  63,  -3,   1,   0,
   0,   1,  14,  31,  17,   1,   0,   0, // SMOOTH
   0,   0,  13,  31,  18,   2,   0,   0,
   0,   0,  11,  31,  20,   2,   0,   0,
   0,   0,  10,  30,  21,   3,   0,   0,
   0,   0,   9,  29,  22,   4,   0,   0,
   0,   0,   8,  28,  23,   5,   0,   0,
   0,  -1,   8,  27,  24,   6,   0,   0,
   0,  -1,   7,  26,  26,   7,  -1,   0,
   0,   0,   6,  24,  27,   8,  -1,   0,
   0,   0,   5,  23,  28,   8,   0,   0,
   0,   0,   4,  22,  29,   9,   0,   0,
   0,   0,   3,  21,  30,  10,   0,   0,
   0,   0,   2,  20,  31,  11,   0,   0,
   0,   0,   2,  18,  31,  13,   0,   0,
   0,   0,   1,  17,  31,  14,   1,   0,
  -1,   1,  -3,  63,   4,  -1,   1,   0, // SHARP
  -1,   3,  -6,  62,   8,  -3,   2,  -1,
  -1,   4,  -9,  60,  13,  -5,   3,  -1,
  -2,   5, -11,  58,  19,  -7,   3,  -1,
  -2,   5, -11,  54,  24,  -9,   4,  -1,
  -2,   5, -12,  50,  30, -10,   4,  -1,
  -2,   5, -12,  45,  35, -11,   5,  -1,
  -2,   6, -12,  40,  40, -12,   6,  -2,
  -1,   5, -11,  35,  45, -12,   5,  -2,
  -1,   4, -10,  30,  50, -12,   5,  -2,
  -1,   4,  -9,  24,  54, -11,   5,  -2,
  -1,   3,  -7,  19,  58, -11,   5,  -2,
  -1,   3,  -5,  13,  60,  -9,   4,  -1,
  -1,   2,  -3,   8,  62,  -6,   3,  -1,
   0,   1,  -1,   4,  63,  -3,   1,  -1,
   0,   0,  -2,  63,   4,  -1,   0,   0, // REGULAR 4
   0,   0,  -4,  61,   9,  -2,   0,   0,
   0,   0,  -5,  58,  14,  -3,   0,   0,
   0,   0,  -6,  55,  19,  -4,   0,   0,
   0,   0,  -6,  51,  24,  -5,   0,   0,
   0,   0,  -7,  47,  29,  -5,   0,   0,
   0,   0,  -6,  42,  33,  -5,   0,   0,
   0,   0,  -6,  38,  38,  -6,   0,   0,
   0,   0,  -5,  33,  42,  -6,   0,   0,
   0,   0,  -5,  29,  47,  -7,   0,   0,
   0,   0,  -5,  24,  51,  -6,   0,   0,
   0,   0,  -4,  19,  55,  -6,   0,   0,
   0,   0,  -3,  14,  58,  -5,   0,   0,
   0,   0,  -2,   9,  61,  -4,   0,   0,
   0,   0,  -1,   4,  63,  -2,   0,   0,
   0,   0,  15,  31,  17,   1,   0,   0, // SMOOTH 4
   0,   0,  13,  31,  18,   2,   0,   0,
   0,   0,  11,  31,  20,   2,   0,   0,
   0,   0,  10,  30,  21,   3,   0,   0,
   0,   0,   9,  29,  22,   4,   0,   0,
   0,   0,   8,  28,  23,   5,   0,   0,
   0,   0,   7,  27,  24,   6,   0,   0,
   0,   0,   6,  26,  26,   6,   0,   0,
   0,   0,   6,  24,  27,   7,   0,   0,
   0,   0,   5,  23,  28,   8,   0,   0,
   0,   0,   4,  22,  29,   9,   0,   0,
   0,   0,   3,  21,  30,  10,   0,   0,
   0,   0,   2,  20,  31,  11,   0,   0,
   0,   0,   2,  18,  31,  13,   0,   0,
   0,   0,   1,  17,  31,  15,   0,   0
]);

#[rustfmt::skip]
#[no_mangle]
pub static rav1e_filter_intra_taps: AlignedArray<[[i8; 64]; 5]> = AlignedArray::new([
  [ -6,  10,  -5,   2,  -3,   1,  -3,   1,
    -4,   6,  -3,   2,  -3,   2,  -3,   1,
     0,   0,  10,   0,   1,  10,   1,   2,
     0,   0,   6,   0,   2,   6,   2,   2,
     0,  12,   0,   9,   0,   7,  10,   5,
     0,   2,   0,   2,   0,   2,   6,   3,
     0,   0,   0,   0,   0,   0,   0,   0,
    12,   0,   9,   0,   7,   0,   5,   0],
  [-10,  16,  -6,   0,  -4,   0,  -2,   0,
   -10,  16,  -6,   0,  -4,   0,  -2,   0,
     0,   0,  16,   0,   0,  16,   0,   0,
     0,   0,  16,   0,   0,  16,   0,   0,
     0,  10,   0,   6,   0,   4,  16,   2,
     0,   0,   0,   0,   0,   0,  16,   0,
     0,   0,   0,   0,   0,   0,   0,   0,
    10,   0,   6,   0,   4,   0,   2,   0],
  [ -8,   8,  -8,   0,  -8,   0,  -8,   0,
    -4,   4,  -4,   0,  -4,   0,  -4,   0,
     0,   0,   8,   0,   0,   8,   0,   0,
     0,   0,   4,   0,   0,   4,   0,   0,
     0,  16,   0,  16,   0,  16,   8,  16,
     0,   0,   0,   0,   0,   0,   4,   0,
     0,   0,   0,   0,   0,   0,   0,   0,
    16,   0,  16,   0,  16,   0,  16,   0],
  [ -2,   8,  -1,   3,  -1,   2,   0,   1,
    -1,   4,  -1,   3,  -1,   2,  -1,   2,
     0,   0,   8,   0,   3,   8,   2,   3,
     0,   0,   4,   0,   3,   4,   2,   3,
     0,  10,   0,   6,   0,   4,   8,   2,
     0,   3,   0,   4,   0,   4,   4,   3,
     0,   0,   0,   0,   0,   0,   0,   0,
    10,   0,   6,   0,   4,   0,   3,   0],
  [-12,  14, -10,   0,  -9,   0,  -8,   0,
   -10,  12,  -9,   1,  -8,   0,  -7,   0,
     0,   0,  14,   0,   0,  14,   0,   0,
     0,   0,  12,   0,   0,  12,   0,   1,
     0,  14,   0,  12,   0,  11,  14,  10,
     0,   0,   0,   0,   0,   1,  12,   1,
     0,   0,   0,   0,   0,   0,   0,   0,
    14,   0,  12,   0,  11,   0,   9,   0]
]);

#[rustfmt::skip]
#[no_mangle]
pub static rav1e_sgr_x_by_xplus1: AlignedArray<[i32; 256]> = AlignedArray::new([
  1,   128, 171, 192, 205, 213, 219, 224, 228, 230, 233, 235, 236, 238, 239,
  240, 241, 242, 243, 243, 244, 244, 245, 245, 246, 246, 247, 247, 247, 247,
  248, 248, 248, 248, 249, 249, 249, 249, 249, 250, 250, 250, 250, 250, 250,
  250, 251, 251, 251, 251, 251, 251, 251, 251, 251, 251, 252, 252, 252, 252,
  252, 252, 252, 252, 252, 252, 252, 252, 252, 252, 252, 252, 252, 253, 253,
  253, 253, 253, 253, 253, 253, 253, 253, 253, 253, 253, 253, 253, 253, 253,
  253, 253, 253, 253, 253, 253, 253, 253, 253, 253, 253, 253, 254, 254, 254,
  254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254,
  254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254,
  254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254,
  254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254,
  254, 254, 254, 254, 254, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
  255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
  255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
  255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
  255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
  255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
  256,
]);

#[rustfmt::skip]
#[no_mangle]
pub static rav1e_mc_warp_filter: AlignedArray<[[i8; 8]; 193]> = AlignedArray::new([
  // [-1, 0)
  [ 0, 127,   0, 0,   0,   1, 0, 0 ], [ 0, 127,   0, 0,  -1,   2, 0, 0 ],
  [ 1, 127,  -1, 0,  -3,   4, 0, 0 ], [ 1, 126,  -2, 0,  -4,   6, 1, 0 ],
  [ 1, 126,  -3, 0,  -5,   8, 1, 0 ], [ 1, 125,  -4, 0,  -6,  11, 1, 0 ],
  [ 1, 124,  -4, 0,  -7,  13, 1, 0 ], [ 2, 123,  -5, 0,  -8,  15, 1, 0 ],
  [ 2, 122,  -6, 0,  -9,  18, 1, 0 ], [ 2, 121,  -6, 0, -10,  20, 1, 0 ],
  [ 2, 120,  -7, 0, -11,  22, 2, 0 ], [ 2, 119,  -8, 0, -12,  25, 2, 0 ],
  [ 3, 117,  -8, 0, -13,  27, 2, 0 ], [ 3, 116,  -9, 0, -13,  29, 2, 0 ],
  [ 3, 114, -10, 0, -14,  32, 3, 0 ], [ 3, 113, -10, 0, -15,  35, 2, 0 ],
  [ 3, 111, -11, 0, -15,  37, 3, 0 ], [ 3, 109, -11, 0, -16,  40, 3, 0 ],
  [ 3, 108, -12, 0, -16,  42, 3, 0 ], [ 4, 106, -13, 0, -17,  45, 3, 0 ],
  [ 4, 104, -13, 0, -17,  47, 3, 0 ], [ 4, 102, -14, 0, -17,  50, 3, 0 ],
  [ 4, 100, -14, 0, -17,  52, 3, 0 ], [ 4,  98, -15, 0, -18,  55, 4, 0 ],
  [ 4,  96, -15, 0, -18,  58, 3, 0 ], [ 4,  94, -16, 0, -18,  60, 4, 0 ],
  [ 4,  91, -16, 0, -18,  63, 4, 0 ], [ 4,  89, -16, 0, -18,  65, 4, 0 ],
  [ 4,  87, -17, 0, -18,  68, 4, 0 ], [ 4,  85, -17, 0, -18,  70, 4, 0 ],
  [ 4,  82, -17, 0, -18,  73, 4, 0 ], [ 4,  80, -17, 0, -18,  75, 4, 0 ],
  [ 4,  78, -18, 0, -18,  78, 4, 0 ], [ 4,  75, -18, 0, -17,  80, 4, 0 ],
  [ 4,  73, -18, 0, -17,  82, 4, 0 ], [ 4,  70, -18, 0, -17,  85, 4, 0 ],
  [ 4,  68, -18, 0, -17,  87, 4, 0 ], [ 4,  65, -18, 0, -16,  89, 4, 0 ],
  [ 4,  63, -18, 0, -16,  91, 4, 0 ], [ 4,  60, -18, 0, -16,  94, 4, 0 ],
  [ 3,  58, -18, 0, -15,  96, 4, 0 ], [ 4,  55, -18, 0, -15,  98, 4, 0 ],
  [ 3,  52, -17, 0, -14, 100, 4, 0 ], [ 3,  50, -17, 0, -14, 102, 4, 0 ],
  [ 3,  47, -17, 0, -13, 104, 4, 0 ], [ 3,  45, -17, 0, -13, 106, 4, 0 ],
  [ 3,  42, -16, 0, -12, 108, 3, 0 ], [ 3,  40, -16, 0, -11, 109, 3, 0 ],
  [ 3,  37, -15, 0, -11, 111, 3, 0 ], [ 2,  35, -15, 0, -10, 113, 3, 0 ],
  [ 3,  32, -14, 0, -10, 114, 3, 0 ], [ 2,  29, -13, 0,  -9, 116, 3, 0 ],
  [ 2,  27, -13, 0,  -8, 117, 3, 0 ], [ 2,  25, -12, 0,  -8, 119, 2, 0 ],
  [ 2,  22, -11, 0,  -7, 120, 2, 0 ], [ 1,  20, -10, 0,  -6, 121, 2, 0 ],
  [ 1,  18,  -9, 0,  -6, 122, 2, 0 ], [ 1,  15,  -8, 0,  -5, 123, 2, 0 ],
  [ 1,  13,  -7, 0,  -4, 124, 1, 0 ], [ 1,  11,  -6, 0,  -4, 125, 1, 0 ],
  [ 1,   8,  -5, 0,  -3, 126, 1, 0 ], [ 1,   6,  -4, 0,  -2, 126, 1, 0 ],
  [ 0,   4,  -3, 0,  -1, 127, 1, 0 ], [ 0,   2,  -1, 0,   0, 127, 0, 0 ],
  // [0, 1)
  [  0,   0,   1, 0, 0, 127,   0,  0 ], [  0,  -1,   2, 0, 0, 127,   0,  0 ],
  [  0,  -3,   4, 1, 1, 127,  -2,  0 ], [  0,  -5,   6, 1, 1, 127,  -2,  0 ],
  [  0,  -6,   8, 1, 2, 126,  -3,  0 ], [ -1,  -7,  11, 2, 2, 126,  -4, -1 ],
  [ -1,  -8,  13, 2, 3, 125,  -5, -1 ], [ -1, -10,  16, 3, 3, 124,  -6, -1 ],
  [ -1, -11,  18, 3, 4, 123,  -7, -1 ], [ -1, -12,  20, 3, 4, 122,  -7, -1 ],
  [ -1, -13,  23, 3, 4, 121,  -8, -1 ], [ -2, -14,  25, 4, 5, 120,  -9, -1 ],
  [ -1, -15,  27, 4, 5, 119, -10, -1 ], [ -1, -16,  30, 4, 5, 118, -11, -1 ],
  [ -2, -17,  33, 5, 6, 116, -12, -1 ], [ -2, -17,  35, 5, 6, 114, -12, -1 ],
  [ -2, -18,  38, 5, 6, 113, -13, -1 ], [ -2, -19,  41, 6, 7, 111, -14, -2 ],
  [ -2, -19,  43, 6, 7, 110, -15, -2 ], [ -2, -20,  46, 6, 7, 108, -15, -2 ],
  [ -2, -20,  49, 6, 7, 106, -16, -2 ], [ -2, -21,  51, 7, 7, 104, -16, -2 ],
  [ -2, -21,  54, 7, 7, 102, -17, -2 ], [ -2, -21,  56, 7, 8, 100, -18, -2 ],
  [ -2, -22,  59, 7, 8,  98, -18, -2 ], [ -2, -22,  62, 7, 8,  96, -19, -2 ],
  [ -2, -22,  64, 7, 8,  94, -19, -2 ], [ -2, -22,  67, 8, 8,  91, -20, -2 ],
  [ -2, -22,  69, 8, 8,  89, -20, -2 ], [ -2, -22,  72, 8, 8,  87, -21, -2 ],
  [ -2, -21,  74, 8, 8,  84, -21, -2 ], [ -2, -22,  77, 8, 8,  82, -21, -2 ],
  [ -2, -21,  79, 8, 8,  79, -21, -2 ], [ -2, -21,  82, 8, 8,  77, -22, -2 ],
  [ -2, -21,  84, 8, 8,  74, -21, -2 ], [ -2, -21,  87, 8, 8,  72, -22, -2 ],
  [ -2, -20,  89, 8, 8,  69, -22, -2 ], [ -2, -20,  91, 8, 8,  67, -22, -2 ],
  [ -2, -19,  94, 8, 7,  64, -22, -2 ], [ -2, -19,  96, 8, 7,  62, -22, -2 ],
  [ -2, -18,  98, 8, 7,  59, -22, -2 ], [ -2, -18, 100, 8, 7,  56, -21, -2 ],
  [ -2, -17, 102, 7, 7,  54, -21, -2 ], [ -2, -16, 104, 7, 7,  51, -21, -2 ],
  [ -2, -16, 106, 7, 6,  49, -20, -2 ], [ -2, -15, 108, 7, 6,  46, -20, -2 ],
  [ -2, -15, 110, 7, 6,  43, -19, -2 ], [ -2, -14, 111, 7, 6,  41, -19, -2 ],
  [ -1, -13, 113, 6, 5,  38, -18, -2 ], [ -1, -12, 114, 6, 5,  35, -17, -2 ],
  [ -1, -12, 116, 6, 5,  33, -17, -2 ], [ -1, -11, 118, 5, 4,  30, -16, -1 ],
  [ -1, -10, 119, 5, 4,  27, -15, -1 ], [ -1,  -9, 120, 5, 4,  25, -14, -2 ],
  [ -1,  -8, 121, 4, 3,  23, -13, -1 ], [ -1,  -7, 122, 4, 3,  20, -12, -1 ],
  [ -1,  -7, 123, 4, 3,  18, -11, -1 ], [ -1,  -6, 124, 3, 3,  16, -10, -1 ],
  [ -1,  -5, 125, 3, 2,  13,  -8, -1 ], [ -1,  -4, 126, 2, 2,  11,  -7, -1 ],
  [  0,  -3, 126, 2, 1,   8,  -6,  0 ], [  0,  -2, 127, 1, 1,   6,  -5,  0 ],
  [  0,  -2, 127, 1, 1,   4,  -3,  0 ], [  0,   0, 127, 0, 0,   2,  -1,  0 ],
  // [1, 2)
  [ 0, 0, 127,   0, 0,   1,   0, 0 ], [ 0, 0, 127,   0, 0,  -1,   2, 0 ],
  [ 0, 1, 127,  -1, 0,  -3,   4, 0 ], [ 0, 1, 126,  -2, 0,  -4,   6, 1 ],
  [ 0, 1, 126,  -3, 0,  -5,   8, 1 ], [ 0, 1, 125,  -4, 0,  -6,  11, 1 ],
  [ 0, 1, 124,  -4, 0,  -7,  13, 1 ], [ 0, 2, 123,  -5, 0,  -8,  15, 1 ],
  [ 0, 2, 122,  -6, 0,  -9,  18, 1 ], [ 0, 2, 121,  -6, 0, -10,  20, 1 ],
  [ 0, 2, 120,  -7, 0, -11,  22, 2 ], [ 0, 2, 119,  -8, 0, -12,  25, 2 ],
  [ 0, 3, 117,  -8, 0, -13,  27, 2 ], [ 0, 3, 116,  -9, 0, -13,  29, 2 ],
  [ 0, 3, 114, -10, 0, -14,  32, 3 ], [ 0, 3, 113, -10, 0, -15,  35, 2 ],
  [ 0, 3, 111, -11, 0, -15,  37, 3 ], [ 0, 3, 109, -11, 0, -16,  40, 3 ],
  [ 0, 3, 108, -12, 0, -16,  42, 3 ], [ 0, 4, 106, -13, 0, -17,  45, 3 ],
  [ 0, 4, 104, -13, 0, -17,  47, 3 ], [ 0, 4, 102, -14, 0, -17,  50, 3 ],
  [ 0, 4, 100, -14, 0, -17,  52, 3 ], [ 0, 4,  98, -15, 0, -18,  55, 4 ],
  [ 0, 4,  96, -15, 0, -18,  58, 3 ], [ 0, 4,  94, -16, 0, -18,  60, 4 ],
  [ 0, 4,  91, -16, 0, -18,  63, 4 ], [ 0, 4,  89, -16, 0, -18,  65, 4 ],
  [ 0, 4,  87, -17, 0, -18,  68, 4 ], [ 0, 4,  85, -17, 0, -18,  70, 4 ],
  [ 0, 4,  82, -17, 0, -18,  73, 4 ], [ 0, 4,  80, -17, 0, -18,  75, 4 ],
  [ 0, 4,  78, -18, 0, -18,  78, 4 ], [ 0, 4,  75, -18, 0, -17,  80, 4 ],
  [ 0, 4,  73, -18, 0, -17,  82, 4 ], [ 0, 4,  70, -18, 0, -17,  85, 4 ],
  [ 0, 4,  68, -18, 0, -17,  87, 4 ], [ 0, 4,  65, -18, 0, -16,  89, 4 ],
  [ 0, 4,  63, -18, 0, -16,  91, 4 ], [ 0, 4,  60, -18, 0, -16,  94, 4 ],
  [ 0, 3,  58, -18, 0, -15,  96, 4 ], [ 0, 4,  55, -18, 0, -15,  98, 4 ],
  [ 0, 3,  52, -17, 0, -14, 100, 4 ], [ 0, 3,  50, -17, 0, -14, 102, 4 ],
  [ 0, 3,  47, -17, 0, -13, 104, 4 ], [ 0, 3,  45, -17, 0, -13, 106, 4 ],
  [ 0, 3,  42, -16, 0, -12, 108, 3 ], [ 0, 3,  40, -16, 0, -11, 109, 3 ],
  [ 0, 3,  37, -15, 0, -11, 111, 3 ], [ 0, 2,  35, -15, 0, -10, 113, 3 ],
  [ 0, 3,  32, -14, 0, -10, 114, 3 ], [ 0, 2,  29, -13, 0,  -9, 116, 3 ],
  [ 0, 2,  27, -13, 0,  -8, 117, 3 ], [ 0, 2,  25, -12, 0,  -8, 119, 2 ],
  [ 0, 2,  22, -11, 0,  -7, 120, 2 ], [ 0, 1,  20, -10, 0,  -6, 121, 2 ],
  [ 0, 1,  18,  -9, 0,  -6, 122, 2 ], [ 0, 1,  15,  -8, 0,  -5, 123, 2 ],
  [ 0, 1,  13,  -7, 0,  -4, 124, 1 ], [ 0, 1,  11,  -6, 0,  -4, 125, 1 ],
  [ 0, 1,   8,  -5, 0,  -3, 126, 1 ], [ 0, 1,   6,  -4, 0,  -2, 126, 1 ],
  [ 0, 0,   4,  -3, 0,  -1, 127, 1 ], [ 0, 0,   2,  -1, 0,   0, 127, 0 ],
  // dummy (replicate row index 191)
  [ 0, 0,   2,  -1, 0,   0, 127, 0 ]
]);

#[rustfmt::skip]
#[no_mangle]
pub static rav1e_dr_intra_derivative: &[i16; 90] = &[
  // More evenly spread out angles and limited to 10-bit
  // Values that are 0 will never be used
     0, 0, 0,       // Approx angle
  1023, 0, 0,       // 3, ...
   547, 0, 0,       // 6, ...
   372, 0, 0, 0, 0, // 9, ...
   273, 0, 0,       // 14, ...
   215, 0, 0,       // 17, ...
   178, 0, 0,       // 20, ...
   151, 0, 0,       // 23, ... (113 & 203 are base angles)
   132, 0, 0,       // 26, ...
   116, 0, 0,       // 29, ...
   102, 0, 0, 0,    // 32, ...
    90, 0, 0,       // 36, ...
    80, 0, 0,       // 39, ...
    71, 0, 0,       // 42, ...
    64, 0, 0,       // 45, ... (45 & 135 are base angles)
    57, 0, 0,       // 48, ...
    51, 0, 0,       // 51, ...
    45, 0, 0, 0,    // 54, ...
    40, 0, 0,       // 58, ...
    35, 0, 0,       // 61, ...
    31, 0, 0,       // 64, ...
    27, 0, 0,       // 67, ... (67 & 157 are base angles)
    23, 0, 0,       // 70, ...
    19, 0, 0,       // 73, ...
    15, 0, 0, 0, 0, // 76, ...
    11, 0, 0,       // 81, ...
     7, 0, 0,       // 84, ...
     3, 0, 0,       // 87, ...
];