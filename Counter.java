public class Counter {
    int key;
    Counter left, right;

    public Counter(int item) {
        this.key = item;
        this.left = null;
        this.right = null;
    }

    public void it(Counter a) {
        this.key = a.key;
    }

    public int set(int a, int b) {
        this.key = a + b;

        return this.key;
    }

    public static void main(String[] args) {
        Counter c = new Counter(3);
        Counter d = new Counter(3);
        c.key = 5;
        d.key = 2;

        c.it(d);

        d.key = 7;
    }
}
