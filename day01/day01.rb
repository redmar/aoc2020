# tried to do it with one line on purpose :D

# one-liner part 1
puts "part 1:"
puts IO.read("input.txt")
		.split("\n")
		.map(&:to_i)
		.combination(2)
		.select { |pair| pair[0] + pair[1] == 2020 }
		.flatten
		.inject(:*)

# one-liner part 2
puts "part 2:"
puts IO.read("input.txt")
		.split("\n")
		.map(&:to_i)
		.combination(3)
		.select { |pair| pair[0] + pair[1] + pair[2] == 2020 }
		.flatten
		.inject(:*)