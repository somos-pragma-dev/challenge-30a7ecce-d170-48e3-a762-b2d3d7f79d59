# Desarrollo de una API REST en Rust con Actix Web y Diesel ORM

Tu tarea es desarrollar una API REST que gestione solicitudes de préstamos en una plataforma de préstamos en línea. La plataforma necesita una API que pueda manejar la creación, lectura, actualización y eliminación de solicitudes de préstamos. La API debe ser idempotente y capaz de manejar un alto volumen de solicitudes, con un throughput esperado de 1 500 solicitudes por segundo en hora pico.

## Informacion General

| Campo | Valor |
|-------|-------|
| **Tema** | rust-actix-web |
| **Nivel** | junior-l1 |
| **Tipo** | practical |
| **Tiempo estimado** | 8 horas |

## Fases del Reto

### Fase 0: Configuración del Proyecto

**Objetivo:** Obtener el proyecto base funcional enviando el Código Base a un asistente de IA, que lo analizará, corregirá errores y generará un ZIP listo para usar.

**Tiempo estimado:** 15-30 minutos

**Instrucciones:**

- Asegúrate de tener instalado para ejecutar el proyecto: Un IDE o editor de código.
- Copia todo el contenido del campo **Código Base** de este reto — incluyendo el texto de instrucciones que aparece al inicio.
- Abre un asistente de IA (Claude en claude.ai, ChatGPT o Gemini — se recomienda Claude), pega el contenido copiado en el chat y envíalo.
- El asistente analizará los archivos, corregirá errores y generará un archivo ZIP descargable. Descárgalo y extráelo en la carpeta donde quieras trabajar.
- Verifica que el proyecto arranca sin errores.

**Entregable:** El proyecto compila/arranca sin errores.

<details>
<summary>Pistas de conocimiento</summary>

- Copia el Código Base completo incluyendo el texto de instrucciones al inicio — esas instrucciones le indican al asistente exactamente qué hacer con los archivos.
- Si el asistente no genera el ZIP automáticamente al terminar el análisis, escríbele: "genera el ZIP ahora".
- Si el proyecto tiene errores al arrancar, comparte el mensaje de error con el mismo asistente para que lo corrija.

</details>

### Fase 1: Definición del Modelo de Datos

**Objetivo:** Definir el modelo de datos para las solicitudes de préstamos.

**Tiempo estimado:** 2 horas

**Instrucciones:**

- Identificar los campos necesarios para una solicitud de préstamo (nombre del solicitante, monto del préstamo, fecha de solicitud, estado de la solicitud).
- Definir las reglas de validación para cada campo (por ejemplo, el monto del préstamo debe ser positivo, el nombre del solicitante no puede estar vacío).
- Establecer las relaciones entre las solicitudes de préstamos y otros recursos del sistema (por ejemplo, el historial de pagos del solicitante).

**Entregable:** Modelo de datos para solicitudes de préstamos, incluyendo campos, reglas de validación y relaciones.

<details>
<summary>Pistas de conocimiento</summary>

- Considera los diferentes tipos de datos y sus restricciones.
- Piensa en cómo las solicitudes de préstamos interactúan con otros componentes del sistema.

</details>

### Fase 2: Implementación de los Endpoints

**Objetivo:** Implementar los endpoints para la creación, lectura, actualización y eliminación de solicitudes de préstamos.

**Tiempo estimado:** 4 horas

**Instrucciones:**

- Crear un endpoint para la creación de una nueva solicitud de préstamo.
- Crear un endpoint para la lectura de una solicitud de préstamo por ID.
- Crear un endpoint para la actualización de una solicitud de préstamo.
- Crear un endpoint para la eliminación de una solicitud de préstamo.
- Asegurar que los endpoints sean idempotentes y manejen correctamente los errores.

**Entregable:** Endpoints implementados para la gestión de solicitudes de préstamos, con manejo idempotente de errores.

<details>
<summary>Pistas de conocimiento</summary>

- Recuerda que la idempotencia significa que múltiples invocaciones con los mismos parámetros deben producir el mismo resultado.
- Considera cómo manejarías errores comunes, como una solicitud con un monto negativo.

</details>

### Fase 3: Optimización y Escalabilidad

**Objetivo:** Optimizar la API para manejar un alto volumen de solicitudes y asegurar su escalabilidad.

**Tiempo estimado:** 2 horas

**Instrucciones:**

- Identificar posibles puntos de cuello de botella en la API.
- Implementar estrategias de optimización para mejorar el rendimiento.
- Asegurar que la API pueda escalar horizontalmente para manejar un mayor número de solicitudes.
- Evaluar el impacto de las optimizaciones en la idempotencia y la consistencia de los datos.

**Entregable:** API optimizada y escalable, con evaluación del impacto de las optimizaciones.

<details>
<summary>Pistas de conocimiento</summary>

- Considera el uso de caché para reducir la carga en la base de datos.
- Piensa en cómo podrías distribuir la carga entre múltiples instancias de la API.

</details>

## Dimensiones Evaluadas

- **queEs**: ¿Qué es una solicitud de préstamo y cuáles son sus componentes esenciales?
- **paraQueSirve**: ¿Para qué sirve cada endpoint en la gestión de solicitudes de préstamos?
- **comoSeUsa**: ¿Cómo se usa la idempotencia en los endpoints de la API?
- **erroresComunes**: ¿Cuáles son los errores comunes que pueden ocurrir al manejar solicitudes de préstamos y cómo se manejan?
- **queDecisionesImplica**: ¿Qué decisiones implica la optimización y escalabilidad de la API?

## Criterios de Evaluacion

- Definición correcta del modelo de datos para solicitudes de préstamos.
- Implementación de endpoints idempotentes y con manejo de errores.
- Optimización y escalabilidad de la API para manejar un alto volumen de solicitudes.

---

*Reto generado automaticamente por Challenge Generator - Pragma*
