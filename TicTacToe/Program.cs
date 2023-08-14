using TicTacToe.Model;

namespace TicTacToe;

public class Program
{
  public static bool CalculateTurn(List<char> field)
  {
    List<List<int>>patterns = new()
    {
      new() {0, 1, 2},
      new() {3, 4, 5},
      new() {6, 7, 8},
      new() {0, 4, 8},
      new() {2, 4, 6},
      new() {0, 3, 6},
      new() {1, 4, 7},
      new() {2, 5, 8},
    };

    return patterns.Exists(p => p.DistinctBy(u => field[u]).Count() == 1);
  }

  public static void Render(List<char> field, Player player)
  {
    Console.WriteLine($"{field[0]} | {field[1]} | {field[2]} \t\t turn {player.Name}");
    Console.WriteLine($"{field[3]} | {field[4]} | {field[5]}");
    Console.WriteLine($"{field[6]} | {field[7]} | {field[8]}");
  }

  public static void Main()
  {
    bool isDone = false;

    do
    {
      Console.WriteLine("1.Play\n2.Quit");

      char key = Console.ReadKey(true).KeyChar;

      switch (Console.ReadKey(true).KeyChar)
      {
        case '1':
          isDone = true;
          break;
        case '2':
          return;

          default:
          Console.WriteLine("Hmm try again");
          Console.ReadKey(true);
          break;
      }
    } while (!isDone);

    List<char> Field = "123456789".ToCharArray().ToList();

    Player p1 = new("John", 'X');
    Player p2 = new("Nica", 'O');
    Player curr = new();

    for (int i = 0; i < 9; i++)
    {
      Console.Clear();

      curr = i % 2 == 0 ? p1 : p2;
      Render(Field, curr);

      byte input;
      byte.TryParse(Console.ReadKey(true).KeyChar.ToString(), out input);

      if (input > 0)
      {
        if (Field[input - 1] == Convert.ToChar(input.ToString()))
        {
          Field[input - 1] = curr.Skin;
        }
      }
      else
      {
        i--;
      }

      if (CalculateTurn(Field))
      {
        Console.Clear();
        Render(Field, curr);

        Console.WriteLine($"{curr.Name} win");
        Console.ReadKey(true);
        return;
      }
    }
  }
}
