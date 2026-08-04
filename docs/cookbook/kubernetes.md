# Kubernetes Deployment Recipe

## Namespace

```yaml
apiVersion: v1
kind: Namespace
metadata:
  name: kcm
```

## ConfigMap

```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: kcm-config
  namespace: kcm
data:
  RUST_LOG: "info"
  KCM_DATA_PATH: "/data/kcm.db"
  KCM_BIND_ADDR: "0.0.0.0:8080"
```

## Secret

```yaml
apiVersion: v1
kind: Secret
metadata:
  name: kcm-secrets
  namespace: kcm
type: Opaque
data:
  encryption-key: <base64-encoded-key>
```

## StatefulSet

```yaml
apiVersion: apps/v1
kind: StatefulSet
metadata:
  name: kcm-server
  namespace: kcm
spec:
  serviceName: kcm-service
  replicas: 1
  selector:
    matchLabels:
      app: kcm-server
  template:
    metadata:
      labels:
        app: kcm-server
    spec:
      containers:
      - name: kcm-server
        image: kcm:latest
        ports:
        - containerPort: 8080
          name: http
        envFrom:
        - configMapRef:
            name: kcm-config
        resources:
          requests:
            memory: "512Mi"
            cpu: "500m"
          limits:
            memory: "2Gi"
            cpu: "2000m"
        volumeMounts:
        - name: data
          mountPath: /data
        livenessProbe:
          httpGet:
            path: /health
            port: 8080
          initialDelaySeconds: 10
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /health
            port: 8080
          initialDelaySeconds: 5
          periodSeconds: 5
  volumeClaimTemplates:
  - metadata:
      name: data
    spec:
      accessModes: ["ReadWriteOnce"]
      resources:
        requests:
          storage: 100Gi
```

## Service

```yaml
apiVersion: v1
kind: Service
metadata:
  name: kcm-service
  namespace: kcm
spec:
  selector:
    app: kcm-server
  ports:
  - name: http
    port: 8080
    targetPort: 8080
  type: LoadBalancer
```

## Deployment

```bash
# Apply all
kubectl apply -f namespace.yaml
kubectl apply -f configmap.yaml
kubectl apply -f secret.yaml
kubectl apply -f statefulset.yaml
kubectl apply -f service.yaml

# Check status
kubectl get pods -n kcm
kubectl logs -f kcm-server-0 -n kcm
```
