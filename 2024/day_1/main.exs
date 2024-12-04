IO.puts("Advent of Code 2024 - Day 1")

defmodule Advent do
  def read_input(file) do
    with {:ok, content} <- File.read(file) do
      content
    else
      {:error, reason} ->
        IO.puts("Error reading file: #{reason}")
        ""
    end
  end

  def process(content) do
    content
    |> String.split("\n", trim: true)
    |> Enum.map(&String.replace(&1, " ", ""))
    |> Enum.map(&split_and_convert/1)
    |> Enum.reduce({[], []}, fn {left, right}, {lefts, rights} ->
      {[left | lefts], [right | rights]}
    end)
  end

  defp split_and_convert(line) do
    {left, right} = String.split_at(line, div(String.length(line), 2))
    {String.to_integer(left), String.to_integer(right)}
  end

  def sort_and_calculate({lefts, rights}) do
    [Enum.sort(lefts), Enum.sort(rights)]
    |> Enum.zip()
    |> Enum.map(fn {a, b} -> abs(a - b) end)
    |> Enum.sum()
  end

  def calculate_occurrences({lefts, rights}) do
    right_freq = Enum.frequencies(rights)

    Enum.map(lefts, fn num -> num * Map.get(right_freq, num, 0) end)
    |> Enum.sum()
  end
end

# Main execution
processed_data =
  "input.txt"
  |> Advent.read_input()
  |> Advent.process()

IO.puts("Absolute Difference Sum: #{Advent.sort_and_calculate(processed_data)}")
IO.puts("Occurrence Sum: #{Advent.calculate_occurrences(processed_data)}")

defmodule AOC_Task_1 do
  @moduledoc """
  Solution for Advent of Code 2024 - Day 1
  Processes input file by splitting numbers and performing calculations
  """

  @doc """
  Read and process the input file

  Returns a tuple of sorted left and right number lists
  """
  def process_input(filename) do
    filename
    |> read_file()
    |> parse_lines()
    |> split_and_convert()
    |> sort_lists()
  end

  @doc """
  Read file with robust error handling
  """
  def read_file(filename) do
    case File.read(filename) do
      {:ok, content} ->
        content

      {:error, reason} ->
        raise "Error reading file #{filename}: #{:file.format_error(reason)}"
    end
  end

  @doc """
  Parse lines, removing empty and whitespace-only lines
  """
  def parse_lines(content) do
    content
    |> String.split("\n", trim: true)
    |> Enum.map(&String.replace(&1, " ", ""))
    |> Enum.reject(&(String.trim(&1) == ""))
  end

  @doc """
  Split lines in half and convert to integers
  """
  def split_and_convert(lines) do
    lines
    |> Enum.map(fn line ->
      mid = round(String.length(line) / 2)
      {String.slice(line, 0, mid), String.slice(line, mid..-1//1)}
    end)
    |> Enum.reduce({[], []}, fn {left, right}, {lefts, rights} ->
      {[String.to_integer(left) | lefts], [String.to_integer(right) | rights]}
    end)
  end

  @doc """
  Sort the left and right number lists
  """
  def sort_lists({lefts, rights}) do
    {Enum.sort(lefts), Enum.sort(rights)}
  end

  @doc """
  Calculate sum of absolute differences
  """
  def calculate_difference({lefts, rights}) do
    lefts
    |> Enum.zip(rights)
    |> Enum.map(fn {a, b} -> abs(a - b) end)
    |> Enum.sum()
  end

  @doc """
  Calculate sum of occurrences
  """
  def calculate_occurrence_sum({lefts, rights}) do
    right_freq = Enum.frequencies(rights)

    lefts
    |> Enum.map(fn num -> num * Map.get(right_freq, num, 0) end)
    |> Enum.sum()
  end

  @doc """
  Main execution function
  """
  def run(filename \\ "input.txt") do
    IO.puts("Advent of Code 2024 - Day 1")

    result =
      filename
      |> process_input()
      |> tap(fn processed_data ->
        IO.puts("Absolute Difference Sum: #{calculate_difference(processed_data)}")
        IO.puts("Occurrence Sum: #{calculate_occurrence_sum(processed_data)}")
      end)

    result
  end
end

# Run the solution
AOC_Task_1.run()
