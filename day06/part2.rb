puts File.read("input.txt")
		.split("\n\n")
		.map { |x| x.split("\n") }
		.map { |x| x.map(&:chars).inject { |acc, x| acc & x }}
		.map(&:count)
		.sum
