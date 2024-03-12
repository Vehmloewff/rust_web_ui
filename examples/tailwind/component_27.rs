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
		ctx.styles(&[Style::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |mut props| {
			props.styles(&[Style::Noop("bg-white"), Style::PaddingY(Size::Exact(96)), Style::OnScreen(Screen::Small, &[Style::PaddingY(Size::Exact(128))])]);
			
			props.child("1", Dynamic).run("div", |mut props| {
				props.styles(&[Style::Noop("mx-auto"), Style::Noop("max-w-7xl"), Style::PaddingX(Size::Exact(24)), Style::OnScreen(Screen::Large, &[Style::PaddingX(Size::Exact(32))])]);
				
				props.child("1", Dynamic).run("dl", |mut props| {
					props.styles(&[Style::Noop("grid"), Style::Noop("grid-cols-1"), Style::Noop("gap-x-8"), Style::Noop("gap-y-16"), Style::Noop("text-center"), Style::OnScreen(Screen::Large, &[Style::Noop("grid-cols-3")])]);
					
					props.child("1", Dynamic).run("div", |mut props| {
						props.styles(&[Style::Noop("mx-auto"), Style::Flex, Style::Noop("max-w-xs"), Style::Noop("flex-col"), Style::Noop("gap-y-4")]);
						
						props.child("1", Dynamic).run("dt", |mut props| {
							props.styles(&[Style::Noop("text-base"), Style::Noop("leading-7"), Style::TextColor(Color::Fg(67))]);
							
							props.child("0", Label).run(|props| props.text("Transactions every 24 hours"));
						});
						props.child("3", Dynamic).run("dd", |mut props| {
							props.styles(&[Style::Noop("order-first"), Style::Noop("text-3xl"), Style::FontSemibold, Style::Noop("tracking-tight"), Style::TextColor(Color::Fg(100)), Style::OnScreen(Screen::Small, &[Style::Noop("text-5xl")])]);
							
							props.child("0", Label).run(|props| props.text("44 million"));
						});
					});
					props.child("3", Dynamic).run("div", |mut props| {
						props.styles(&[Style::Noop("mx-auto"), Style::Flex, Style::Noop("max-w-xs"), Style::Noop("flex-col"), Style::Noop("gap-y-4")]);
						
						props.child("1", Dynamic).run("dt", |mut props| {
							props.styles(&[Style::Noop("text-base"), Style::Noop("leading-7"), Style::TextColor(Color::Fg(67))]);
							
							props.child("0", Label).run(|props| props.text("Assets under holding"));
						});
						props.child("3", Dynamic).run("dd", |mut props| {
							props.styles(&[Style::Noop("order-first"), Style::Noop("text-3xl"), Style::FontSemibold, Style::Noop("tracking-tight"), Style::TextColor(Color::Fg(100)), Style::OnScreen(Screen::Small, &[Style::Noop("text-5xl")])]);
							
							props.child("0", Label).run(|props| props.text("$119 trillion"));
						});
					});
					props.child("5", Dynamic).run("div", |mut props| {
						props.styles(&[Style::Noop("mx-auto"), Style::Flex, Style::Noop("max-w-xs"), Style::Noop("flex-col"), Style::Noop("gap-y-4")]);
						
						props.child("1", Dynamic).run("dt", |mut props| {
							props.styles(&[Style::Noop("text-base"), Style::Noop("leading-7"), Style::TextColor(Color::Fg(67))]);
							
							props.child("0", Label).run(|props| props.text("New users annually"));
						});
						props.child("3", Dynamic).run("dd", |mut props| {
							props.styles(&[Style::Noop("order-first"), Style::Noop("text-3xl"), Style::FontSemibold, Style::Noop("tracking-tight"), Style::TextColor(Color::Fg(100)), Style::OnScreen(Screen::Small, &[Style::Noop("text-5xl")])]);
							
							props.child("0", Label).run(|props| props.text("46,000"));
						});
					});
				});
			});
		});
	}
}
