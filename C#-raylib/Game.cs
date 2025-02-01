using Raylib_cs;
using System.Numerics;

public class Game
{
    public const int window_width = 800;
    public const int window_height = 480;

    public class Paddle
    {
        public const int padding = 50;
        public const int width= 10;
        public const int height = 100;
        public const int speed = 4;
        public int y = window_height/2-height/2;
    }

    public class Ball
    {
        public const int radius = 10;
        public float speed = 3f;
        public float angle = 0;
        public Vector2 position = new(window_width / 2, window_height / 2);
    }

    public static bool IsXOutsideScreen(int x, int width)
    {
        return x < 0 || x + width > window_width;
    }

    static int Main(string[] args)
    {
        Raylib.SetConfigFlags(ConfigFlags.Msaa4xHint | ConfigFlags.VSyncHint);
        Raylib.InitWindow(window_width, window_height, "PING PONG");

        Paddle p1 = new();
        Paddle p2 = new();
        Ball ball = new();

        Raylib.SetTargetFPS(60);

        while (!Raylib.WindowShouldClose())
        {
            Raylib.BeginDrawing();
            Raylib.ClearBackground(new(32, 32, 54, 255));

            //Player
            Raylib.DrawRectangle(Paddle.padding, p1.y, Paddle.width, Paddle.height, Color.White);

            if (Raylib.IsKeyDown(KeyboardKey.Up)) p1.y -= Paddle.speed;
            if (Raylib.IsKeyDown(KeyboardKey.Down)) p1.y += Paddle.speed;

            if (p1.y < 0) p1.y = 0;
            if (p1.y > window_height - Paddle.height) p1.y = window_height - Paddle.height;

            //Bot
            Raylib.DrawRectangle(window_width-Paddle.width-Paddle.padding, p2.y, Paddle.width, Paddle.height, Color.White);

            if (ball.position.Y < p2.y + 20)
            {
                p2.y -= Paddle.speed;
            }
            else if (ball.position.Y > p2.y + Paddle.height - 20)
            {
                p2.y += Paddle.speed;
            }

            if (IsXOutsideScreen((int)ball.position.X, Ball.radius))
            {
                ball = new();
            }

            if (p2.y < 0) p2.y = 0;
            if (p2.y > window_height - Paddle.height) p2.y = window_height - Paddle.height;

            //Ball
            Raylib.DrawRectangle((int)ball.position.X, (int)ball.position.Y, Ball.radius, Ball.radius, Color.White);

            if (ball.position.Y < 0 || ball.position.Y + Ball.radius > window_height) ball.angle *= -1;

            if (Raylib.CheckCollisionRecs(new(Paddle.padding, p1.y, Paddle.width, Paddle.height), new(ball.position.X, ball.position.Y, Ball.radius, Ball.radius)))
            {
                ball.speed += 0.3f;
                ball.angle += 180 - ((p1.y + Paddle.height / 2f - ball.position.Y) * 0.4f);
            }

            if (Raylib.CheckCollisionRecs(new(window_width - Paddle.width - Paddle.padding, p2.y, Paddle.width, Paddle.height), new(ball.position.X, ball.position.Y, Ball.radius, Ball.radius)))
            {
                ball.speed += 0.3f;
                ball.angle += 180 - ((p2.y + Paddle.height / 2f - ball.position.Y) * 0.4f);
            }
         
            ball.position.X += ball.speed * MathF.Cos(ball.angle * (MathF.PI / 180));
            ball.position.Y += ball.speed * MathF.Sin(ball.angle * (MathF.PI / 180));

            Raylib.EndDrawing();
        }

        Raylib.CloseWindow();

        return 0;
    }
}