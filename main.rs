use dioxus::prelude::*;

fn main() {
    launch(app);
}

fn app() -> Element {
    // -------- Estado base --------
    let mut weight = use_signal(|| 70.0_f32);
    let mut activity = use_signal(|| 1.1_f32);

    // -------- Cálculos base (mínimo factible) --------
    // 22 kcal/kg ≈ gasto basal promedio
    let calories = weight() * 22.0 * activity();

    let protein_g = weight() * 1.6;
    let fat_g = weight() * 0.8;

    let used_kcal = protein_g * 4.0 + fat_g * 9.0;
    let carbs_g = ((calories - used_kcal).max(0.0)) / 4.0;

    let water_ml = weight() * 30.0 + if activity() >= 1.6 { 500.0 } else { 0.0 };

    // -------- Consejos rápidos --------
    let tips = quick_tips(activity());

    rsx! {
        div {
            style: "
                font-family: system-ui, sans-serif;
                max-width: 420px;
                margin: auto;
                padding: 1rem;
                line-height: 1.4;
            ",

            h2 { "Nutrición diaria simple" }

            p {
                style: "font-size: 0.9rem; color: #555;",
                "Esta es una estimación diaria mínima y práctica,
                 pensada como referencia general."
            }

            // -------- Inputs --------
            label { "Peso (kg)" }
            input {
                r#type: "number",
                value: "{weight()}",
                oninput: move |e| {
                    if let Ok(v) = e.value().parse::<f32>() {
                        weight.set(v.max(0.0));
                    }
                }
            }

            br {}
            br {}

            label { "Nivel de actividad" }
            select {
                onchange: move |e| {
                    if let Ok(v) = e.value().parse::<f32>() {
                        activity.set(v);
                    }
                },

                option { value: "1.1", "🛏️ Encamado / sin actividad" }
                option { value: "1.25", "🪑 Muy poca" }
                option { value: "1.4", "🚶 Ligera" }
                option { value: "1.6", "🏃 Activa" }
                option { value: "1.8", "🔥 Muy intensa" }
            }

            hr {}

            // -------- Resultados --------
            h3 { "Referencia diaria aproximada" }

            p { "🔥 Calorías: {calories.round()} kcal" }
            p { "🥩 Proteína: {protein_g.round()} g" }
            p { "🍚 Carbohidratos: {carbs_g.round()} g" }
            p { "🥑 Grasas: {fat_g.round()} g" }
            p { "💧 Agua: {(water_ml / 1000.0):.1} L" }

            p {
                style: "font-size: 0.85rem; color: #555;",
                "Estos valores representan un punto de partida razonable.
                 Las necesidades reales pueden variar."
            }

            hr {}

            // -------- Consejos --------
            h3 { "Consejos rápidos" }

            ul {
                for tip in tips {
                    li { "{tip}" }
                }
            }

            hr {}

            small {
                style: "color: #666;",
                "Información orientativa. No reemplaza asesoría médica o nutricional."
            }
        }
    }
}

// -------- Lógica de consejos (simple y defensiva) --------
fn quick_tips(activity: f32) -> Vec<&'static str> {
    let mut tips = Vec::new();

    if activity <= 1.25 {
        tips.push("🍽️ Mantén horarios de comida regulares.");
        tips.push("🥩 Reparte la proteína a lo largo del día.");
    }

    if activity >= 1.4 {
        tips.push("🍚 Incluye carbohidratos cerca de tu actividad.");
    }

    if activity >= 1.6 {
        tips.push("💧 Aumenta líquidos si sudas o te ejercitas.");
    }

    tips.push("🥑 Prioriza grasas saludables como aceite de oliva.");
    tips.push("🚫 Limita frituras y grasas saturadas.");

    tips
}
