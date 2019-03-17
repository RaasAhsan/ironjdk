public class Counter {

    private int i = 0;

    public Counter(int initial) {
        this.i = initial;
    }

    public int get() {
        return this.i;
    }

    public void increment() {
        this.i = this.i + 1;
    }

    public void decrement() {
        this.i = this.i - 1;
    }

    public void add(int d) {
        this.i += d;
    }

}
