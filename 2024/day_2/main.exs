defmodule Advent do
  def process_file(filename) do
    case File.read(filename) do
      {:ok, content} ->
        content
        |> String.split("\n", trim: true)
        |> Enum.map(&String.split/1)
        |> Enum.map(fn line -> Enum.map(line, &String.to_integer/1) end)

      {:error, reason} ->
        IO.puts("Error reading file: #{reason}")
        []
    end
  end

  def is_safe(list) do
    case list do
      [] -> false
      [_] -> true
      [a, b | _tail] when a > b -> is_safe_greater(list)
      [a, b | _tail] when a < b -> is_safe_smaller(list)
      _ -> false
    end
  end

  def is_safe_greater(list) do
    case list do
      [_] -> true
      [a, b | tail] when a > b and abs(a - b) <= 3 -> is_safe_greater([b | tail])
      _ -> false
    end
  end

  def is_safe_smaller(list) do
    case list do
      [_] -> true
      [a, b | tail] when a < b and abs(b - a) <= 3 -> is_safe_smaller([b | tail])
      _ -> false
    end
  end

  def check?(array) do
    case array do
      [] ->
        false

      [_] ->
        true

      _ ->
        increasing =
          Enum.chunk_every(array, 2, 1, :discard)
          |> Enum.all?(fn [a, b] -> a < b and abs(b - a) <= 3 end)

        decreasing =
          Enum.chunk_every(array, 2, 1, :discard)
          |> Enum.all?(fn [a, b] -> a > b and abs(a - b) <= 3 end)

        increasing or decreasing
    end
  end

  def check_all?(array) do
    do_check(array, 0, array)
  end

  defp do_check(array, skipped, original) when skipped < length(original) do
    modified = List.delete_at(array, skipped)

    cond do
      check?(modified) -> true
      skipped == length(original) - 1 -> false
      true -> do_check(original, skipped + 1, original)
    end
  end

  def run_part_1(filename \\ "input_test.txt") do
    process_file(filename)
    |> IO.inspect()
    |> Enum.map(&is_safe(&1))
    |> IO.inspect()
  end

  def run_part_2(filename \\ "input_test.txt") do
    process_file(filename)
    |> Enum.map(&check_all?/1)
    |> Enum.count(& &1)
    |> IO.inspect()
  end
end

IO.puts("Advent of Code 2024 - Day 2")
IO.puts("Advent of Code 2024 - Day 2 - Part 1")
Advent.run_part_1()
IO.puts("Advent of Code 2024 - Day 2 - Part 2")
Advent.run_part_2()
Advent.run_part_2("input.txt")
