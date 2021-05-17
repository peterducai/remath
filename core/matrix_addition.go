package core

func MatrixInt64Addition(x []int64, y []int64) []int64 {
	//var z []int64 = x + y
	var z []int64 = append(x, y...)
	return z
}
