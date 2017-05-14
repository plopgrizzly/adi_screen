# 
# adi_screen - Aldaron's Device Interface - Screen - "gen-spv.sh"
# Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
# 

GlslangValidator=glslangValidator

./$GlslangValidator glsl/color.frag -V -o src/res/color-frag.spv
./$GlslangValidator glsl/color.vert -V -o src/res/color-vert.spv

./$GlslangValidator glsl/texture.frag -V -o src/res/texture-frag.spv
./$GlslangValidator glsl/texture.vert -V -o src/res/texture-vert.spv
