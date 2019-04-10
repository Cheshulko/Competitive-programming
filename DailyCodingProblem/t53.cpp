#include <iostream>
#include <stack>

using std::cin;
using std::count;

template <typename T>
class Queue
{
  public:
    void enqueue(const T &t)
    {
        head.push(t);
    }
    T dequeue()
    {
        if (!tail.empty())
        {
            T top = tail.top();
            tail.pop();
            return top;
        }
        else
        {
            while (!head.empty())
            {
                T top = head.top();
                head.pop();
                tail.push(top);
            }
            if (!tail.empty())
            {
                T top = tail.top();
                tail.pop();
                return top;
            }
        }
    }

  private:
    std::stack<T> head;
    std::stack<T> tail;
};

int main()
{
    Queue<double> q;
    q.enqueue(10.1);
    q.enqueue(12.1);

    std::cout << q.dequeue() << std::endl;

    q.enqueue(42.1);

    std::cout << q.dequeue() << std::endl;

    q.enqueue(43.1);

    std::cout << q.dequeue() << std::endl;
    std::cout << q.dequeue() << std::endl;

    return 0;
}

// Implement a queue using two stacks. Recall that a queue is a FIFO (first-in, first-out) data structure with the following methods: enqueue, which inserts an element into the queue, and dequeue, which removes it.