defmodule Advent do
  @letters ["X", "M", "A", "S"]

  @directions [
    {0, 1},
    {1, 0},
    {0, -1},
    {-1, 0},
    {1, 1},
    {1, -1},
    {-1, 1},
    {-1, -1}
  ]

  def process_file(filename) do
    case File.read(filename) do
      {:ok, content} ->
        content
        |> String.split("\n", trim: true)
        |> Enum.map(&String.split(&1, ""))
        |> Enum.map(fn x -> Enum.filter(x, fn y -> byte_size(y) > 0 end) end)

      {:error, reason} ->
        IO.puts("Error reading file: #{reason}")
        []
    end
  end

  def get_next_letter(letter) do
    case Enum.find_index(@letters, fn x -> x == letter end) do
      nil ->
        Enum.at(@letters, 0)

      index ->
        Enum.at(@letters, index + 1)
    end
  end

  def start_looking(rows, row_index) do
    count = 0
    look_for_x(count, rows, row_index)
  end

  def look_for_x(count, rows, row_index) do
    if row_index >= Enum.count(rows) do
      count
    end

    row = rows |> Enum.at(row_index)

    case Enum.find_index(row, fn x -> x == "X" end) do
      nil ->
        look_for_x(count, rows, row_index + 1)

      column_index ->
        Enum.each(@directions, fn direction ->
          if look_for_letter(rows, row_index, column_index, direction, "M") do
            count = count + 1
            look_for_x(count, rows, row_index + 1)
          end
        end)

        count
    end
  end

  def look_for_letter(rows, row_index, column_index, direction, letter) do
    {row, column} = direction
    row_index = row_index + row
    column_index = column_index + column

    case Enum.at(rows, row_index) do
      nil ->
        false

      row ->
        case Enum.at(row, column_index) do
          nil ->
            false

          l ->
            if l == letter do
              if l == "S" do
                true
              else
                look_for_letter(
                  rows,
                  row_index,
                  column_index,
                  direction,
                  get_next_letter(letter)
                )
              end
            end
        end
    end
  end

  def run(filename \\ "input_test.txt") do
    process_file(filename)
    |> start_looking(0)
    |> IO.inspect()
  end
end

IO.puts("Advent of Code 2024 - Day 3")
Advent.run()
# Advent.run("input.txt")
