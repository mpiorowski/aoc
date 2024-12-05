defmodule Advent do
  def process_file(filename) do
    case File.read(filename) do
      {:ok, content} ->
        content

      {:error, reason} ->
        IO.puts("Error reading file: #{reason}")
        []
    end
  end

  def split_by_command(original, command) do
    original
    |> String.split(command, parts: 2)
  end

  def extract_valid(original, results, on) do
    command = if on, do: "do()", else: "don't()"

    case split_by_command(original, command) do
      [first, second] ->
        results = if !on, do: results ++ [first], else: results
        extract_valid(second, results, !on)

      [first] ->
        results = if !on, do: results ++ [first], else: results
        results

      [] ->
        results
    end
  end

  def concat_array(array) do
    Enum.reduce(array, "", fn x, acc -> acc <> x end)
  end

  def extract_mul(content) do
    content
    |> String.split("mul(")
    |> Enum.slice(1..-1//1)
    |> Enum.filter(&String.contains?(&1, ")"))
    |> Enum.flat_map(&String.split(&1, ")"))
    |> Enum.filter(&String.contains?(&1, ","))
    |> Enum.map(&String.split(&1, ","))
    |> Enum.filter(&(length(&1) == 2))
  end

  def parse_numbers(content) do
    content
    |> Enum.map(fn [a, b] -> {parse_to_int(a), parse_to_int(b)} end)
  end

  def parse_to_int(x) do
    case Integer.parse(x) do
      {num, ""} -> num
      _ -> 0
    end
  end

  def multiple_tuples(tuples) do
    tuples
    |> Enum.map(fn {a, b} -> a * b end)
  end

  def run(filename \\ "input_test.txt") do
    process_file(filename)
    |> IO.inspect()
    |> extract_valid([], false)
    |> IO.inspect()
    |> concat_array()
    |> extract_mul()
    |> parse_numbers()
    |> multiple_tuples()
    |> Enum.sum()
    |> IO.inspect()
  end
end

IO.puts("Advent of Code 2024 - Day 3")
Advent.run()
Advent.run("input.txt")
