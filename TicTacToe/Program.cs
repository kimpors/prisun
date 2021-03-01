using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace TicTacToe
{
    class Program
    {
        static void Main(string[] args)
        {
            char[] field = { '1', '2', '3', '4', '5', '6', '7', '8', '9' };

            // Знак для игрока
            const char X = 'X';

            for (int i = 0; i < 9; i++)
            {
                Console.Clear();

                // Вивод поля
                Console.WriteLine($"{field[0]} | {field[1]} | {field[2]}");
                Console.WriteLine($"{field[3]} | {field[4]} | {field[5]}");
                Console.WriteLine($"{field[6]} | {field[7]} | {field[8]}");

                byte result;
                byte.TryParse(Console.ReadKey(true).KeyChar.ToString(), out result);

                if (result > 0)
                {
                    field[result - 1] = X;
                }
            }
        }
    }
}
