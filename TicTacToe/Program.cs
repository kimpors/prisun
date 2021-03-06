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
        static void PrintField(char[] field)
        {
            Console.WriteLine($"{field[0]} | {field[1]} | {field[2]}");
            Console.WriteLine($"{field[3]} | {field[4]} | {field[5]}");
            Console.WriteLine($"{field[6]} | {field[7]} | {field[8]}");

        }

        static void Main(string[] args)
        {
            char[] field = { '1', '2', '3', '4', '5', '6', '7', '8', '9' };


            Player P1 = new Player(0, 'X', "John");
            Player P2 = new Player(0, 'O', "Nica");


            for (int i = 0; i < 9; i++)
            {
                Console.Clear();
                PrintField(field);


                byte input;
                byte.TryParse(Console.ReadKey(true).KeyChar.ToString(), out input);


                if (input > 0)
                {
                    if (field[input - 1] == Convert.ToChar(input.ToString()))
                    {
                        field[input - 1] = i % 2 == 0 ? P1.Symbol : P2.Symbol;
                    }
                    else
                    {
                        i--;
                    }
                }





                // Логіка вийграша 
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
                    PrintField(field);

                    Console.WriteLine("You win");
                    Console.ReadKey(true);
                    return;
                }
            }
        }
    }
}
