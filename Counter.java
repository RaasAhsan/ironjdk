public class Counter {
    int key;
    Counter left, right;

    public Counter(int item) {
        this.key = item;
        this.left = null;
        this.right = null;
    }
}
