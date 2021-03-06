using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading;

namespace TicTacToe
{

    struct Player
    {
        public int Score { get; set; }
        public char Symbol { get; set; }
        public string Name { get; set; }


       public Player(int score, char symbol , string name)
        {
            Score = score;
            Symbol = symbol;
            Name = name;
        }

    }



    
    class Program
    {


        // Вивод поля
        static void PrintField(char[] field, Player player)
        {
            Console.WriteLine($"{field[0]} | {field[1]} | {field[2]} \t\t step {player.Name}");
            Console.WriteLine($"{field[3]} | {field[4]} | {field[5]}");
            Console.WriteLine($"{field[6]} | {field[7]} | {field[8]}");

        }
           
        static void Main(string[] args)
        {
            // Меню

            bool isDone = false;
            do
            {
                Console.WriteLine("1.Play\n2.Log\n3.Quit");
                string choise = Console.ReadKey(true).KeyChar.ToString();

                switch (choise)
                {
                    case "1":
                        isDone = true;
                        break;

                    case "3":
                        return;

                    default:
                        Console.WriteLine("Hmmm Try again");
                        Console.ReadKey(true);
                        break;
                }
            } while (!isDone);

            //Ігра

            char[] field = { '1', '2', '3', '4', '5', '6', '7', '8', '9' };


            Player P1 = new Player(0, 'X', "John");
            Player P2 = new Player(0, 'O', "Nica");
            Player playerStep = new Player(); // Показивает хто ходить

            for (int i = 0; i < 9; i++)
            {
                Console.Clear();


                playerStep = i % 2 == 0 ? P1 : P2;


                PrintField(field, playerStep);


                byte input;
                byte.TryParse(Console.ReadKey(true).KeyChar.ToString(), out input);

                //Ето для того чтоби не можна було замінити уже поставлений знак
                if (input > 0)
                {
                    if (field[input - 1] == Convert.ToChar(input.ToString()))
                    {
                        field[input - 1] = playerStep.Symbol;
                    }
                    else
                    {
                        i--;
                    }
                }
              


                // Логіка вийграша 
                //Да да я знаю  громосткій кусок кода
                if (field[0] == field[1] && field[1] == field[2] ||
                    field[3] == field[4] && field[4] == field[5] ||
                    field[6] == field[7] && field[7] == field[8] ||
                    field[0] == field[4] && field[4] == field[8] ||
                    field[2] == field[4] && field[4] == field[6] ||
                    field[0] == field[3] && field[3] == field[6] ||
                    field[1] == field[4] && field[4] == field[7] ||
                    field[2] == field[5] && field[5] == field[8] )
                {
                    Console.Clear();
                    PrintField(field,playerStep);
                    Console.WriteLine($"{playerStep.Name} win");


                    Console.ReadKey(true);
                    return;
                }
            }
        }
    }
}
