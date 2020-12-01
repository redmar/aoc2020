{:ok, input} = File.read("input.txt")

numbers =
  input
  |> String.split("\n")
  |> Enum.filter(fn line -> String.trim(line) != "" end)
  |> Enum.map(&String.to_integer/1)

solution_part1 =
  for x <- numbers, y <- numbers do
    [x, y]
  end
  |> Enum.find(fn [x, y] -> x + y == 2020 end)
  |> case do
    [x, y] -> x * y
  end

IO.puts("solution for part1 = #{solution_part1}")

solution_part2 =
  for x <- numbers, y <- numbers, z <- numbers do
    [x, y, z]
  end
  |> Enum.find(fn [x, y, z] -> x + y + z == 2020 end)
  |> case do
    [x, y, z] -> x * y * z
  end

IO.puts("solution for part2 = #{solution_part2}")
