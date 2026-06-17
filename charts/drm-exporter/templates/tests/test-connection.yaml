apiVersion: v1
kind: Pod
metadata:
  name: {{ include "drm-exporter.fullname" . }}-test-connection
  labels:
    {{- include "drm-exporter.labels" . | nindent 4 }}
  annotations:
    helm.sh/hook: test
    helm.sh/hook-delete-policy: before-hook-creation,hook-succeeded
spec:
  restartPolicy: Never
  containers:
    - name: curl
      image: {{ include "drm-exporter.testImage" . | quote }}
      imagePullPolicy: {{ .Values.tests.image.pullPolicy }}
      command: ["curl"]
      # Hit /health (the exporter's lightweight liveness endpoint) through the
      # Service, confirming the DaemonSet is up and serving.
      args:
        - --fail
        - --silent
        - --show-error
        - http://{{ include "drm-exporter.fullname" . }}:{{ .Values.config.port }}/health
      securityContext:
        allowPrivilegeEscalation: false
        readOnlyRootFilesystem: true
        runAsNonRoot: true
        runAsUser: 65532
        capabilities:
          drop:
            - ALL
        seccompProfile:
          type: RuntimeDefault
