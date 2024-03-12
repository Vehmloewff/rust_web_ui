use rust_web_ui::*;

pub struct Example42;

pub struct Example42Props {}

impl Default for Example42Props {
	fn default() -> Example42Props {
		Example42Props { }
	}
}

impl Widget<'_> for Example42 {
	type Props = Example42Props;

	fn render(mut ctx: Ctx<'_>, props: Example42Props) {
		ctx.styles(&[NoStyle::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |props| {
			props.styles(&[NoStyle::Noop("bg-white"), Style::PaddingY(96), Screen::Small(&[Style::PaddingY(128)])]);
			
			props.child("1", Dynamic).run("div", |props| {
				props.styles(&[NoStyle::Noop("mx-auto"), NoStyle::Noop("max-w-7xl"), Style::PaddingX(24), Screen::Large(&[Style::PaddingX(32)])]);
				
				props.child("1", Dynamic).run("div", |props| {
					props.styles(&[NoStyle::Noop("mx-auto"), NoStyle::Noop("max-w-2xl"), Screen::Large(&[NoStyle::Noop("text-center")])]);
					
					props.child("1", Dynamic).run("h2", |props| {
						props.styles(&[NoStyle::Noop("text-base"), Style::FontSemibold, NoStyle::Noop("leading-7"), NoStyle::Noop("text-indigo-600")]);
						
						props.child("0", Label).run(|props| props.set_text("Deploy faster"));
					});
					props.child("3", Dynamic).run("p", |props| {
						props.styles(&[Style::MarginTop(8), NoStyle::Noop("text-3xl"), Style::FontBold, NoStyle::Noop("tracking-tight"), Style::TextColor(Color::Fg(100)), Screen::Small(&[NoStyle::Noop("text-4xl")])]);
						
						props.child("0", Label).run(|props| props.set_text("Everything you need to deploy your app"));
					});
					props.child("5", Dynamic).run("p", |props| {
						props.styles(&[Style::MarginTop(24), NoStyle::Noop("text-lg"), NoStyle::Noop("leading-8"), Style::TextColor(Color::Fg(67))]);
						
						props.child("0", Label).run(|props| props.set_text("Quis tellus eget adipiscing convallis sit sit eget aliquet quis. Suspendisse eget egestas a elementum pulvinar et feugiat blandit at. In mi viverra elit nunc."));
					});
				});
				props.child("3", Dynamic).run("div", |props| {
					props.styles(&[NoStyle::Noop("mx-auto"), Style::MarginTop(64), NoStyle::Noop("max-w-2xl"), Screen::Small(&[Style::MarginTop(80)]), Screen::Large(&[Style::MarginTop(96)]), Screen::Large(&[NoStyle::Noop("max-w-4xl")])]);
					
					props.child("1", Dynamic).run("dl", |props| {
						props.styles(&[NoStyle::Noop("grid"), NoStyle::Noop("max-w-xl"), NoStyle::Noop("grid-cols-1"), NoStyle::Noop("gap-x-8"), NoStyle::Noop("gap-y-10"), Screen::Large(&[NoStyle::Noop("max-w-none")]), Screen::Large(&[NoStyle::Noop("grid-cols-2")]), Screen::Large(&[NoStyle::Noop("gap-y-16")])]);
						
						props.child("1", Dynamic).run("div", |props| {
							props.styles(&[NoStyle::Noop("relative"), Style::PaddingLeft(64)]);
							
							props.child("1", Dynamic).run("dt", |props| {
								props.styles(&[NoStyle::Noop("text-base"), Style::FontSemibold, NoStyle::Noop("leading-7"), Style::TextColor(Color::Fg(100))]);
								
								props.child("1", Dynamic).run("div", |props| {
									props.styles(&[NoStyle::Noop("absolute"), NoStyle::Noop("left-0"), NoStyle::Noop("top-0"), Style::Flex, Style::Width(40), Style::Width(40), Style::ItemsCenter, Style::JustifyCenter, NoStyle::Noop("rounded-lg"), NoStyle::Noop("bg-indigo-600")]);
									
									props.child("1", Icon).run(|props| {
										props.style(&[Style::Width(24), Style::Width(24), NoStyle::Noop("text-white")]);
									});
								});
								props.child("2", Label).run(|props| props.set_text("Push to deploy"));
							});
							props.child("3", Dynamic).run("dd", |props| {
								props.styles(&[Style::MarginTop(8), NoStyle::Noop("text-base"), NoStyle::Noop("leading-7"), Style::TextColor(Color::Fg(67))]);
								
								props.child("0", Label).run(|props| props.set_text("Morbi viverra dui mi arcu sed. Tellus semper adipiscing suspendisse semper morbi. Odio urna massa nunc massa."));
							});
						});
						props.child("3", Dynamic).run("div", |props| {
							props.styles(&[NoStyle::Noop("relative"), Style::PaddingLeft(64)]);
							
							props.child("1", Dynamic).run("dt", |props| {
								props.styles(&[NoStyle::Noop("text-base"), Style::FontSemibold, NoStyle::Noop("leading-7"), Style::TextColor(Color::Fg(100))]);
								
								props.child("1", Dynamic).run("div", |props| {
									props.styles(&[NoStyle::Noop("absolute"), NoStyle::Noop("left-0"), NoStyle::Noop("top-0"), Style::Flex, Style::Width(40), Style::Width(40), Style::ItemsCenter, Style::JustifyCenter, NoStyle::Noop("rounded-lg"), NoStyle::Noop("bg-indigo-600")]);
									
									props.child("1", Icon).run(|props| {
										props.style(&[Style::Width(24), Style::Width(24), NoStyle::Noop("text-white")]);
									});
								});
								props.child("2", Label).run(|props| props.set_text("SSL certificates"));
							});
							props.child("3", Dynamic).run("dd", |props| {
								props.styles(&[Style::MarginTop(8), NoStyle::Noop("text-base"), NoStyle::Noop("leading-7"), Style::TextColor(Color::Fg(67))]);
								
								props.child("0", Label).run(|props| props.set_text("Sit quis amet rutrum tellus ullamcorper ultricies libero dolor eget. Sem sodales gravida quam turpis enim lacus amet."));
							});
						});
						props.child("5", Dynamic).run("div", |props| {
							props.styles(&[NoStyle::Noop("relative"), Style::PaddingLeft(64)]);
							
							props.child("1", Dynamic).run("dt", |props| {
								props.styles(&[NoStyle::Noop("text-base"), Style::FontSemibold, NoStyle::Noop("leading-7"), Style::TextColor(Color::Fg(100))]);
								
								props.child("1", Dynamic).run("div", |props| {
									props.styles(&[NoStyle::Noop("absolute"), NoStyle::Noop("left-0"), NoStyle::Noop("top-0"), Style::Flex, Style::Width(40), Style::Width(40), Style::ItemsCenter, Style::JustifyCenter, NoStyle::Noop("rounded-lg"), NoStyle::Noop("bg-indigo-600")]);
									
									props.child("1", Icon).run(|props| {
										props.style(&[Style::Width(24), Style::Width(24), NoStyle::Noop("text-white")]);
									});
								});
								props.child("2", Label).run(|props| props.set_text("Simple queues"));
							});
							props.child("3", Dynamic).run("dd", |props| {
								props.styles(&[Style::MarginTop(8), NoStyle::Noop("text-base"), NoStyle::Noop("leading-7"), Style::TextColor(Color::Fg(67))]);
								
								props.child("0", Label).run(|props| props.set_text("Quisque est vel vulputate cursus. Risus proin diam nunc commodo. Lobortis auctor congue commodo diam neque."));
							});
						});
						props.child("7", Dynamic).run("div", |props| {
							props.styles(&[NoStyle::Noop("relative"), Style::PaddingLeft(64)]);
							
							props.child("1", Dynamic).run("dt", |props| {
								props.styles(&[NoStyle::Noop("text-base"), Style::FontSemibold, NoStyle::Noop("leading-7"), Style::TextColor(Color::Fg(100))]);
								
								props.child("1", Dynamic).run("div", |props| {
									props.styles(&[NoStyle::Noop("absolute"), NoStyle::Noop("left-0"), NoStyle::Noop("top-0"), Style::Flex, Style::Width(40), Style::Width(40), Style::ItemsCenter, Style::JustifyCenter, NoStyle::Noop("rounded-lg"), NoStyle::Noop("bg-indigo-600")]);
									
									props.child("1", Icon).run(|props| {
										props.style(&[Style::Width(24), Style::Width(24), NoStyle::Noop("text-white")]);
									});
								});
								props.child("2", Label).run(|props| props.set_text("Advanced security"));
							});
							props.child("3", Dynamic).run("dd", |props| {
								props.styles(&[Style::MarginTop(8), NoStyle::Noop("text-base"), NoStyle::Noop("leading-7"), Style::TextColor(Color::Fg(67))]);
								
								props.child("0", Label).run(|props| props.set_text("Arcu egestas dolor vel iaculis in ipsum mauris. Tincidunt mattis aliquet hac quis. Id hac maecenas ac donec pharetra eget."));
							});
						});
					});
				});
			});
		});
	}
}
