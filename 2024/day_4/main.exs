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

  # top-left
  # top-right
  # bottom-left
  # bottom-right
  @diagonals [
    {-1, -1},
    {-1, 1},
    {1, -1},
    {1, 1}
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

  @spec get_next_letter(String.t()) :: String.t()
  def get_next_letter(letter) do
    case Enum.find_index(@letters, fn x -> x == letter end) do
      nil ->
        Enum.at(@letters, 0)

      index ->
        Enum.at(@letters, index + 1)
    end
  end

  @spec start_looking(list(list(String.t()))) :: integer()
  def start_looking(rows) do
    rows
    |> Enum.with_index()
    |> Enum.map(fn {row, row_index} ->
      row
      |> Enum.with_index()
      |> Enum.filter(fn {col, _} -> col == "X" end)
      |> Enum.map(fn {_, col_index} ->
        Enum.count(@directions, fn direction ->
          look_for_letter(rows, row_index, col_index, direction, "M")
        end)
      end)
    end)
    |> List.flatten()
    |> Enum.sum()
  end

  def look_for_letter(rows, row_index, column_index, {row_delta, col_delta}, letter) do
    new_row = row_index + row_delta
    new_col = column_index + col_delta

    cond do
      new_row < 0 or new_col < 0 ->
        false

      new_row >= length(rows) or new_col >= length(Enum.at(rows, 0)) ->
        false

      true ->
        case Enum.at(Enum.at(rows, new_row), new_col) do
          ^letter ->
            letter == "S" or
              look_for_letter(
                rows,
                new_row,
                new_col,
                {row_delta, col_delta},
                get_next_letter(letter)
              )

          _ ->
            false
        end
    end
  end

  def look_for_diagonals(rows, row_index, column_index) do
    @diagonals
    |> Enum.map(fn {delta_row, delta_col} ->
      new_row = row_index + delta_row
      new_col = column_index + delta_col

      cond do
        new_row < 0 or new_col < 0 ->
          0

        new_row >= length(rows) ->
          0

        new_col >= length(Enum.at(rows, 0)) ->
          0

        true ->
          rows
          |> Enum.at(new_row)
          |> Enum.at(new_col, 0)
      end
    end)
  end

  def start_looking_2(rows) do
    rows
    |> Enum.with_index()
    |> Enum.map(fn {row, row_index} ->
      row
      |> Enum.with_index()
      |> Enum.filter(fn {col, _} -> col == "A" end)
      |> Enum.map(fn {_, col_index} ->
        look_for_diagonals(rows, row_index, col_index)
      end)
      |> Enum.filter(fn array ->
        Enum.count(array, &(&1 == "M")) == 2 and Enum.count(array, &(&1 == "S")) == 2 and
          Enum.at(array, 0) != Enum.at(array, 3)
      end)
    end)
  end

  def run_part_1(filename \\ "input_test.txt") do
    process_file(filename)
    |> start_looking()
    |> IO.inspect()
  end

  def run_part_2(filename \\ "input_test.txt") do
    process_file(filename)
    |> start_looking_2()
    |> Enum.flat_map(& &1)
    |> IO.inspect()
    |> length()
    |> IO.inspect()
  end
end

IO.puts("Advent of Code 2024 - Day 3")
Advent.run_part_1()
Advent.run_part_1("input.txt")
Advent.run_part_2()
Advent.run_part_2("input.txt")
