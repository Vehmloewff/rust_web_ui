use rust_web_ui::*;

pub struct Example27;

pub struct Example27Props {}

impl Default for Example27Props {
	fn default() -> Example27Props {
		Example27Props { }
	}
}

impl Widget<'_> for Example27 {
	type Props = Example27Props;

	fn render(mut ctx: Ctx<'_>, props: Example27Props) {
		ctx.styles(&[NoStyle::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |props| {
			props.styles(&[NoStyle::Noop("bg-white"), Style::PaddingY(96), Screen::Small(&[Style::PaddingY(128)])]);
			
			props.child("1", Dynamic).run("div", |props| {
				props.styles(&[NoStyle::Noop("mx-auto"), NoStyle::Noop("max-w-7xl"), Style::PaddingX(24), Screen::Large(&[Style::PaddingX(32)])]);
				
				props.child("1", Dynamic).run("dl", |props| {
					props.styles(&[NoStyle::Noop("grid"), NoStyle::Noop("grid-cols-1"), NoStyle::Noop("gap-x-8"), NoStyle::Noop("gap-y-16"), NoStyle::Noop("text-center"), Screen::Large(&[NoStyle::Noop("grid-cols-3")])]);
					
					props.child("1", Dynamic).run("div", |props| {
						props.styles(&[NoStyle::Noop("mx-auto"), Style::Flex, NoStyle::Noop("max-w-xs"), NoStyle::Noop("flex-col"), NoStyle::Noop("gap-y-4")]);
						
						props.child("1", Dynamic).run("dt", |props| {
							props.styles(&[NoStyle::Noop("text-base"), NoStyle::Noop("leading-7"), Style::TextColor(Color::Fg(67))]);
							
							props.child("0", Label).run(|props| props.set_text("Transactions every 24 hours"));
						});
						props.child("3", Dynamic).run("dd", |props| {
							props.styles(&[NoStyle::Noop("order-first"), NoStyle::Noop("text-3xl"), Style::FontSemibold, NoStyle::Noop("tracking-tight"), Style::TextColor(Color::Fg(100)), Screen::Small(&[NoStyle::Noop("text-5xl")])]);
							
							props.child("0", Label).run(|props| props.set_text("44 million"));
						});
					});
					props.child("3", Dynamic).run("div", |props| {
						props.styles(&[NoStyle::Noop("mx-auto"), Style::Flex, NoStyle::Noop("max-w-xs"), NoStyle::Noop("flex-col"), NoStyle::Noop("gap-y-4")]);
						
						props.child("1", Dynamic).run("dt", |props| {
							props.styles(&[NoStyle::Noop("text-base"), NoStyle::Noop("leading-7"), Style::TextColor(Color::Fg(67))]);
							
							props.child("0", Label).run(|props| props.set_text("Assets under holding"));
						});
						props.child("3", Dynamic).run("dd", |props| {
							props.styles(&[NoStyle::Noop("order-first"), NoStyle::Noop("text-3xl"), Style::FontSemibold, NoStyle::Noop("tracking-tight"), Style::TextColor(Color::Fg(100)), Screen::Small(&[NoStyle::Noop("text-5xl")])]);
							
							props.child("0", Label).run(|props| props.set_text("$119 trillion"));
						});
					});
					props.child("5", Dynamic).run("div", |props| {
						props.styles(&[NoStyle::Noop("mx-auto"), Style::Flex, NoStyle::Noop("max-w-xs"), NoStyle::Noop("flex-col"), NoStyle::Noop("gap-y-4")]);
						
						props.child("1", Dynamic).run("dt", |props| {
							props.styles(&[NoStyle::Noop("text-base"), NoStyle::Noop("leading-7"), Style::TextColor(Color::Fg(67))]);
							
							props.child("0", Label).run(|props| props.set_text("New users annually"));
						});
						props.child("3", Dynamic).run("dd", |props| {
							props.styles(&[NoStyle::Noop("order-first"), NoStyle::Noop("text-3xl"), Style::FontSemibold, NoStyle::Noop("tracking-tight"), Style::TextColor(Color::Fg(100)), Screen::Small(&[NoStyle::Noop("text-5xl")])]);
							
							props.child("0", Label).run(|props| props.set_text("46,000"));
						});
					});
				});
			});
		});
	}
}
