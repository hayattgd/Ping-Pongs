#include "/usr/include/raylib.h"
#include <math.h>

#define WINDOW_WIDTH 800
#define WINDOW_HEIGHT 480

typedef struct
{
	int padding;

	int width;
	int height;
	int speed;

	int y;
} paddle;

typedef struct
{
	int radius;

	float speed;
	float angle;
	
	float x;
	float y;
} ball;


int isXOutsideScreen(int x, int width)
{
	return x < 0 || x + width > WINDOW_WIDTH;
}

int main()
{
	SetConfigFlags(FLAG_MSAA_4X_HINT | FLAG_VSYNC_HINT);
	InitWindow(WINDOW_WIDTH, WINDOW_HEIGHT, "PING PONG");

	SetTargetFPS(60);

	paddle p1 = {.padding = 50, .width = 10, .height = 100, .speed = 4, .y = WINDOW_HEIGHT/2-p1.height/2};
	paddle p2 = p1;

	ball default_ball = {.radius = 10, .speed = 3, .angle = 0, .x = WINDOW_WIDTH/2, .y = WINDOW_HEIGHT/2};
	ball ball = default_ball;

	while (!WindowShouldClose())
	{
		BeginDrawing();

		Color bg = {32, 32, 54};
		ClearBackground(bg);

		//Player
		DrawRectangle(p1.padding, p1.y, p1.width, p1.height, WHITE);

		if (IsKeyDown(KEY_UP)) p1.y -= p1.speed;
		if (IsKeyDown(KEY_DOWN)) p1.y += p1.speed;

		if (p1.y < 0) p1.y = 0;
		if (p1.y > WINDOW_HEIGHT - p1.height) p1.y = WINDOW_HEIGHT - p1.height;

		//Bot
		DrawRectangle(WINDOW_WIDTH-p2.width-p2.padding, p2.y, p2.width, p2.height, WHITE);

		if (ball.y < p2.y + 20)
		{
			p2.y -= p2.speed;
		}
		else if (ball.y > p2.y + p2.height - 20)
		{
			p2.y += p2.speed;
		}

		if (isXOutsideScreen((int)ball.x, ball.radius))
		{
			ball = default_ball;
		}

		if (p2.y < 0) p2.y = 0;
		if (p2.y > WINDOW_HEIGHT - p2.height) p2.y = WINDOW_HEIGHT - p2.height;

		//Ball
		DrawRectangle((int)ball.x, (int)ball.y, ball.radius, ball.radius, WHITE);

		if (ball.y < 0 || ball.y + ball.radius > WINDOW_HEIGHT) ball.angle *= -1;

		if (CheckCollisionRecs((Rectangle){.x = p1.padding, .y = p1.y, .width= p1.width, .height = p1.height}, (Rectangle){.x = ball.x, .y = ball.y, .width = ball.radius, .height = ball.radius}))
		{
			ball.speed += 0.3f;
			ball.angle += 180 - ((p1.y + p1.height / 2 - ball.y) * 0.4);
		}

		if (CheckCollisionRecs((Rectangle){.x = WINDOW_WIDTH - p2.width - p2.padding, .y = p2.y, .width= p2.width, .height = p2.height}, (Rectangle){.x = ball.x, .y = ball.y, .width = ball.radius, .height = ball.radius}))
		{
			ball.speed += 0.3f;
			ball.angle += 180 - ((p2.y + p2.height / 2 - ball.y) * 0.4);
		}
	
		ball.x += ball.speed * cosf(ball.angle * (PI / 180));
		ball.y += ball.speed * sinf(ball.angle * (PI / 180));

		EndDrawing();
	}

	CloseWindow();

	return 0;
}
