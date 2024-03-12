use rust_web_ui::*;

pub struct Example15;

pub struct Example15Props {}

impl Default for Example15Props {
	fn default() -> Example15Props {
		Example15Props { }
	}
}

impl Widget<'_> for Example15 {
	type Props = Example15Props;

	fn render(mut ctx: Ctx<'_>, props: Example15Props) {
		ctx.styles(&[NoStyle::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |props| {
			props.styles(&[Style::Flex, Style::ItemsCenter, Style::JustifyCenter, NoStyle::Noop("bg-white"), Style::Padding(32)]);
			
			props.child("1", Dynamic).run("div", |props| {
				props.styles(&[NoStyle::Noop("mx-auto"), NoStyle::Noop("w-full"), NoStyle::Noop("max-w-xs")]);
				
				props.child("1", Dynamic).run("div", |props| {
					props.child("1", Dynamic).run("label", |props| {
						props.set_attribute("for", "price");
						props.styles(&[Style::Block, NoStyle::Noop("text-sm"), NoStyle::Noop("font-medium"), NoStyle::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
						
						props.child("0", Label).run(|props| props.set_text("Price"));
					});
					props.child("3", Dynamic).run("div", |props| {
						props.styles(&[NoStyle::Noop("relative"), Style::MarginTop(8), NoStyle::Noop("rounded-md"), NoStyle::Noop("shadow-sm")]);
						
						props.child("1", Dynamic).run("div", |props| {
							props.styles(&[NoStyle::Noop("pointer-events-none"), NoStyle::Noop("absolute"), NoStyle::Noop("inset-y-0"), NoStyle::Noop("left-0"), Style::Flex, Style::ItemsCenter, Style::PaddingLeft(12)]);
							
							props.child("1", Dynamic).run("span", |props| {
								props.styles(&[Style::TextColor(Color::Fg(56)), Screen::Small(&[NoStyle::Noop("text-sm")])]);
								
								props.child("0", Label).run(|props| props.set_text("$"));
							});
						});
						props.child("3", Dynamic).run("input", |props| {
							props.set_attribute("type", "text");
							props.set_attribute("name", "price");
							props.set_attribute("id", "price");
							props.styles(&[Style::Block, NoStyle::Noop("w-full"), NoStyle::Noop("rounded-md"), NoStyle::Noop("border-0"), Style::PaddingY(6), Style::PaddingLeft(28), Style::PaddingRight(80), Style::TextColor(Color::Fg(100)), NoStyle::Noop("ring-1"), NoStyle::Noop("ring-inset"), NoStyle::Noop("ring-gray-300"), NoStyle::NoopGroup("placeholder", Style::TextColor(Color::Fg(44))), Action::Hover(&[NoStyle::Noop("ring-2")]), Action::Hover(&[NoStyle::Noop("ring-inset")]), Action::Hover(&[NoStyle::Noop("ring-indigo-600")]), Screen::Small(&[NoStyle::Noop("text-sm")]), Screen::Small(&[NoStyle::Noop("leading-6")])]);
							props.set_attribute("placeholder", "0.00");
						});
						props.child("5", Dynamic).run("div", |props| {
							props.styles(&[NoStyle::Noop("absolute"), NoStyle::Noop("inset-y-0"), NoStyle::Noop("right-0"), Style::Flex, Style::ItemsCenter]);
							
							props.child("1", Dynamic).run("label", |props| {
								props.set_attribute("for", "currency");
								props.styles(&[NoStyle::Noop("sr-only")]);
								
								props.child("0", Label).run(|props| props.set_text("Currency"));
							});
							props.child("3", Dynamic).run("select", |props| {
								props.set_attribute("id", "currency");
								props.set_attribute("name", "currency");
								props.styles(&[NoStyle::Noop("h-full"), NoStyle::Noop("rounded-md"), NoStyle::Noop("border-0"), NoStyle::Noop("bg-transparent"), Style::PaddingY(0), Style::PaddingLeft(8), Style::PaddingRight(28), Style::TextColor(Color::Fg(56)), Action::Hover(&[NoStyle::Noop("ring-2")]), Action::Hover(&[NoStyle::Noop("ring-inset")]), Action::Hover(&[NoStyle::Noop("ring-indigo-600")]), Screen::Small(&[NoStyle::Noop("text-sm")])]);
								
								props.child("1", Dynamic).run("option", |props| {
									props.child("0", Label).run(|props| props.set_text("USD"));
								});
								props.child("3", Dynamic).run("option", |props| {
									props.child("0", Label).run(|props| props.set_text("CAD"));
								});
								props.child("5", Dynamic).run("option", |props| {
									props.child("0", Label).run(|props| props.set_text("EUR"));
								});
							});
						});
					});
				});
			});
		});
	}
}
