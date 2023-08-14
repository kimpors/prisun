namespace TicTacToe.Model;

public class Player
{
  public Player() {}

  public Player(string name, char skin)
  {
    Name = name;
    Skin = skin;
  }

  public string Name { get; set; }
  public char Skin { get; set; }
  public int Score { get; set; }
}
