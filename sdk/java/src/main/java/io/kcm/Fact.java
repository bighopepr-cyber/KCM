package io.kcm;

/**
 * Represents a KCM knowledge fact.
 */
public class Fact {
    public final int subject;
    public final byte predicate;
    public final int object;
    public final double confidence;

    public Fact(int subject, byte predicate, int object, double confidence) {
        if (confidence < 0.0 || confidence > 1.0) {
            throw new IllegalArgumentException("Confidence must be in [0, 1], got " + confidence);
        }
        this.subject = subject;
        this.predicate = predicate;
        this.object = object;
        this.confidence = confidence;
    }

    @Override
    public String toString() {
        return String.format("Fact{subject=%d, predicate=%d, object=%d, confidence=%.2f}",
            subject, predicate, object, confidence);
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (!(o instanceof Fact)) return false;
        Fact f = (Fact) o;
        return subject == f.subject && predicate == f.predicate &&
               object == f.object && Math.abs(confidence - f.confidence) < 1e-10;
    }
}
