use rust_web_ui::*;

pub struct Example43;

pub struct Example43Props {}

impl Default for Example43Props {
	fn default() -> Example43Props {
		Example43Props { }
	}
}

impl Widget<'_> for Example43 {
	type Props = Example43Props;

	fn render(mut ctx: Ctx<'_>, props: Example43Props) {
		ctx.styles(&[NoStyle::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |props| {
			props.styles(&[NoStyle::Noop("bg-white"), Style::PaddingY(96), Screen::Small(&[Style::PaddingY(128)])]);
			
			props.child("1", Dynamic).run("div", |props| {
				props.styles(&[NoStyle::Noop("mx-auto"), NoStyle::Noop("max-w-7xl"), Style::PaddingX(24), Screen::Large(&[Style::PaddingX(32)])]);
				
				props.child("1", Dynamic).run("div", |props| {
					props.styles(&[NoStyle::Noop("mx-auto"), NoStyle::Noop("max-w-2xl"), Screen::Small(&[NoStyle::Noop("text-center")])]);
					
					props.child("1", Dynamic).run("h2", |props| {
						props.styles(&[NoStyle::Noop("text-3xl"), Style::FontBold, NoStyle::Noop("tracking-tight"), Style::TextColor(Color::Fg(100)), Screen::Small(&[NoStyle::Noop("text-4xl")])]);
						
						props.child("0", Label).run(|props| props.set_text("Simple no-tricks pricing"));
					});
					props.child("3", Dynamic).run("p", |props| {
						props.styles(&[Style::MarginTop(24), NoStyle::Noop("text-lg"), NoStyle::Noop("leading-8"), Style::TextColor(Color::Fg(67))]);
						
						props.child("0", Label).run(|props| props.set_text("Distinctio et nulla eum soluta et neque labore quibusdam. Saepe et quasi iusto modi velit ut non voluptas in. Explicabo id ut laborum."));
					});
				});
				props.child("3", Dynamic).run("div", |props| {
					props.styles(&[NoStyle::Noop("mx-auto"), Style::MarginTop(64), NoStyle::Noop("max-w-2xl"), NoStyle::Noop("rounded-3xl"), NoStyle::Noop("ring-1"), NoStyle::Noop("ring-gray-200"), Screen::Small(&[Style::MarginTop(80)]), Screen::Large(&[Style::MarginX(0)]), Screen::Large(&[Style::Flex]), Screen::Large(&[NoStyle::Noop("max-w-none")])]);
					
					props.child("1", Dynamic).run("div", |props| {
						props.styles(&[Style::Padding(32), Screen::Small(&[Style::Padding(40)]), Screen::Large(&[NoStyle::Noop("flex-auto")])]);
						
						props.child("1", Dynamic).run("h3", |props| {
							props.styles(&[NoStyle::Noop("text-2xl"), Style::FontBold, NoStyle::Noop("tracking-tight"), Style::TextColor(Color::Fg(100))]);
							
							props.child("0", Label).run(|props| props.set_text("Lifetime membership"));
						});
						props.child("3", Dynamic).run("p", |props| {
							props.styles(&[Style::MarginTop(24), NoStyle::Noop("text-base"), NoStyle::Noop("leading-7"), Style::TextColor(Color::Fg(67))]);
							
							props.child("0", Label).run(|props| props.set_text("Lorem ipsum dolor sit amet consect etur adipisicing elit. Itaque amet indis perferendis blanditiis repellendus etur quidem assumenda."));
						});
						props.child("5", Dynamic).run("div", |props| {
							props.styles(&[Style::MarginTop(40), Style::Flex, Style::ItemsCenter, NoStyle::Noop("gap-x-4")]);
							
							props.child("1", Dynamic).run("h4", |props| {
								props.styles(&[NoStyle::Noop("flex-none"), NoStyle::Noop("text-sm"), Style::FontSemibold, NoStyle::Noop("leading-6"), NoStyle::Noop("text-indigo-600")]);
								
								props.child("0", Label).run(|props| props.set_text("What’s included"));
							});
							props.child("3", Dynamic).run("div", |props| {
								props.styles(&[NoStyle::Noop("h-px"), NoStyle::Noop("flex-auto"), Style::Color(Color::Fg(11))]);
							});
						});
						props.child("7", Dynamic).run("ul", |props| {
							props.set_attribute("role", "list");
							props.styles(&[Style::MarginTop(32), NoStyle::Noop("grid"), NoStyle::Noop("grid-cols-1"), NoStyle::Noop("gap-4"), NoStyle::Noop("text-sm"), NoStyle::Noop("leading-6"), Style::TextColor(Color::Fg(67)), Screen::Small(&[NoStyle::Noop("grid-cols-2")]), Screen::Small(&[NoStyle::Noop("gap-6")])]);
							
							props.child("1", Dynamic).run("li", |props| {
								props.styles(&[Style::Flex, NoStyle::Noop("gap-x-3")]);
								
								props.child("1", Icon).run(|props| {
									props.style(&[Style::Width(24), Style::Width(20), NoStyle::Noop("flex-none"), NoStyle::Noop("text-indigo-600")]);
								});
								props.child("2", Label).run(|props| props.set_text("Private forum access"));
							});
							props.child("3", Dynamic).run("li", |props| {
								props.styles(&[Style::Flex, NoStyle::Noop("gap-x-3")]);
								
								props.child("1", Icon).run(|props| {
									props.style(&[Style::Width(24), Style::Width(20), NoStyle::Noop("flex-none"), NoStyle::Noop("text-indigo-600")]);
								});
								props.child("2", Label).run(|props| props.set_text("Member resources"));
							});
							props.child("5", Dynamic).run("li", |props| {
								props.styles(&[Style::Flex, NoStyle::Noop("gap-x-3")]);
								
								props.child("1", Icon).run(|props| {
									props.style(&[Style::Width(24), Style::Width(20), NoStyle::Noop("flex-none"), NoStyle::Noop("text-indigo-600")]);
								});
								props.child("2", Label).run(|props| props.set_text("Entry to annual conference"));
							});
							props.child("7", Dynamic).run("li", |props| {
								props.styles(&[Style::Flex, NoStyle::Noop("gap-x-3")]);
								
								props.child("1", Icon).run(|props| {
									props.style(&[Style::Width(24), Style::Width(20), NoStyle::Noop("flex-none"), NoStyle::Noop("text-indigo-600")]);
								});
								props.child("2", Label).run(|props| props.set_text("Official member t-shirt"));
							});
						});
					});
					props.child("3", Dynamic).run("div", |props| {
						props.styles(&[NoStyle::Noop("-mt-2"), Style::Padding(8), Screen::Large(&[Style::MarginTop(0)]), Screen::Large(&[NoStyle::Noop("w-full")]), Screen::Large(&[NoStyle::Noop("max-w-md")]), Screen::Large(&[NoStyle::Noop("flex-shrink-0")])]);
						
						props.child("1", Dynamic).run("div", |props| {
							props.styles(&[NoStyle::Noop("rounded-2xl"), Style::Color(Color::Fg(6)), Style::PaddingY(40), NoStyle::Noop("text-center"), NoStyle::Noop("ring-1"), NoStyle::Noop("ring-inset"), NoStyle::Noop("ring-gray-900/5"), Screen::Large(&[Style::Flex]), Screen::Large(&[NoStyle::Noop("flex-col")]), Screen::Large(&[Style::JustifyCenter]), Screen::Large(&[Style::PaddingY(64)])]);
							
							props.child("1", Dynamic).run("div", |props| {
								props.styles(&[NoStyle::Noop("mx-auto"), NoStyle::Noop("max-w-xs"), Style::PaddingX(32)]);
								
								props.child("1", Dynamic).run("p", |props| {
									props.styles(&[NoStyle::Noop("text-base"), Style::FontSemibold, Style::TextColor(Color::Fg(67))]);
									
									props.child("0", Label).run(|props| props.set_text("Pay once, own it forever"));
								});
								props.child("3", Dynamic).run("p", |props| {
									props.styles(&[Style::MarginTop(24), Style::Flex, NoStyle::Noop("items-baseline"), Style::JustifyCenter, NoStyle::Noop("gap-x-2")]);
									
									props.child("1", Dynamic).run("span", |props| {
										props.styles(&[NoStyle::Noop("text-5xl"), Style::FontBold, NoStyle::Noop("tracking-tight"), Style::TextColor(Color::Fg(100))]);
										
										props.child("0", Label).run(|props| props.set_text("$349"));
									});
									props.child("3", Dynamic).run("span", |props| {
										props.styles(&[NoStyle::Noop("text-sm"), Style::FontSemibold, NoStyle::Noop("leading-6"), NoStyle::Noop("tracking-wide"), Style::TextColor(Color::Fg(67))]);
										
										props.child("0", Label).run(|props| props.set_text("USD"));
									});
								});
								props.child("5", Dynamic).run("a", |props| {
									props.set_attribute("href", "#");
									props.styles(&[Style::MarginTop(40), Style::Block, NoStyle::Noop("w-full"), NoStyle::Noop("rounded-md"), NoStyle::Noop("bg-indigo-600"), Style::PaddingX(12), Style::PaddingY(8), NoStyle::Noop("text-center"), NoStyle::Noop("text-sm"), Style::FontSemibold, NoStyle::Noop("text-white"), NoStyle::Noop("shadow-sm"), Action::Hover(&[NoStyle::Noop("bg-indigo-500")]), NoStyle::NoopGroup("focus-visible", NoStyle::Noop("outline")), NoStyle::NoopGroup("focus-visible", NoStyle::Noop("outline-2")), NoStyle::NoopGroup("focus-visible", NoStyle::Noop("outline-offset-2")), NoStyle::NoopGroup("focus-visible", NoStyle::Noop("outline-indigo-600"))]);
									
									props.child("0", Label).run(|props| props.set_text("Get access"));
								});
								props.child("7", Dynamic).run("p", |props| {
									props.styles(&[Style::MarginTop(24), NoStyle::Noop("text-xs"), NoStyle::Noop("leading-5"), Style::TextColor(Color::Fg(67))]);
									
									props.child("0", Label).run(|props| props.set_text("Invoices and receipts available for easy company reimbursement"));
								});
							});
						});
					});
				});
			});
		});
	}
}
