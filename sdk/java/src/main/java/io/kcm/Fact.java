package io.kcm;

public final class Fact {
    public final int subject;
    public final byte predicate;
    public final int object;
    public final double confidence;
    public final byte evidence;
    public final long timestamp;
    public final byte context;
    public final int version;
    public final byte priority;
    public final short owner;

    public Fact(int subject, byte predicate, int object, double confidence,
                byte evidence, long timestamp, byte context, int version,
                byte priority, short owner) {
        if (confidence < 0.0 || confidence > 1.0) {
            throw new IllegalArgumentException("Confidence must be in [0, 1], got " + confidence);
        }
        this.subject = subject;
        this.predicate = predicate;
        this.object = object;
        this.confidence = confidence;
        this.evidence = evidence;
        this.timestamp = timestamp;
        this.context = context;
        this.version = version;
        this.priority = priority;
        this.owner = owner;
    }

    public Fact(int subject, byte predicate, int object, double confidence) {
        this(subject, predicate, object, confidence, (byte) 0, 0L, (byte) 0, 0, (byte) 0, (short) 0);
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (!(o instanceof Fact)) return false;
        Fact f = (Fact) o;
        return subject == f.subject && predicate == f.predicate && object == f.object &&
               Double.compare(confidence, f.confidence) == 0 && evidence == f.evidence &&
               timestamp == f.timestamp && context == f.context && version == f.version &&
               priority == f.priority && owner == f.owner;
    }

    @Override
    public int hashCode() {
        int h = 17;
        h = 31 * h + subject;
        h = 31 * h + predicate;
        h = 31 * h + object;
        h = 31 * h + Double.hashCode(confidence);
        h = 31 * h + evidence;
        h = 31 * h + Long.hashCode(timestamp);
        h = 31 * h + context;
        h = 31 * h + version;
        h = 31 * h + priority;
        h = 31 * h + owner;
        return h;
    }

    @Override
    public String toString() {
        return String.format("Fact{subject=%d, predicate=%d, object=%d, confidence=%.2f, evidence=%d, timestamp=%d, context=%d, version=%d, priority=%d, owner=%d}",
            subject, predicate, object, confidence, evidence, timestamp, context, version, priority, owner);
    }
}
