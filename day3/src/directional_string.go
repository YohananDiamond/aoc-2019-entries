// Code generated by "stringer -type=Directional"; DO NOT EDIT.

package main

import "strconv"

func _() {
	// An "invalid array index" compiler error signifies that the constant values have changed.
	// Re-run the stringer command to generate them again.
	var x [1]struct{}
	_ = x[VOID-0]
	_ = x[LEFT-1]
	_ = x[RIGHT-2]
	_ = x[UP-3]
	_ = x[DOWN-4]
}

const _Directional_name = "VOIDLEFTRIGHTUPDOWN"

var _Directional_index = [...]uint8{0, 4, 8, 13, 15, 19}

func (i Directional) String() string {
	if i < 0 || i >= Directional(len(_Directional_index)-1) {
		return "Directional(" + strconv.FormatInt(int64(i), 10) + ")"
	}
	return _Directional_name[_Directional_index[i]:_Directional_index[i+1]]
}
